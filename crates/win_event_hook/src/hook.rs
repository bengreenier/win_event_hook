use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Weak},
};

use lazy_static::lazy_static;
use rayon::{ThreadPool, ThreadPoolBuilder};
use tracing::{debug, trace, warn};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    System::Threading::GetCurrentThreadId,
    UI::{
        Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK},
        WindowsAndMessaging::{PostThreadMessageW, WM_QUIT},
    },
};

use crate::{
    config::Config,
    errors::{Error, Result},
    event_loop::run_event_loop,
    events::Event,
    handler::{EventHandler, WindowHandle},
};

pub trait WinEventHookInner: Sync + Send {
    fn installed(&self) -> bool;
    fn uninstall(&mut self) -> Result<()>;
}

pub struct UnthreadedInner {
    handle: Option<HWINEVENTHOOK>,
    _config: Config,
    _handler: Arc<(Box<dyn EventHandler>, Option<Vec<Event>>)>,
}

impl UnthreadedInner {
    pub fn new(config: Config, handler: Box<dyn EventHandler>) -> Result<Self> {
        let handle = unsafe {
            SetWinEventHook(
                config.event_min,
                config.event_max,
                config.module_handle.unwrap_or_default(),
                Some(__on_win_event_hook_event),
                config.id_process,
                config.id_thread,
                config.dw_flags.bits(),
            )
        };

        trace!(?handle, "installed hook");

        let handler = Arc::new((handler, config.event_filter.clone()));

        // block-scoped write-lock for INSTALLED_HOOKS
        {
            // A failure here indicates a library issue. Please open an issue on GitHub!
            let mut hooks = INSTALLED_HOOKS
                .write()
                .expect("Unable to obtain write lock");

            hooks.insert(handle.0, Arc::downgrade(&handler));
        }

        trace!("write hook weakref into storage");

        Ok(Self {
            handle: Some(handle),
            _config: config,
            _handler: handler,
        })
    }
}

impl WinEventHookInner for UnthreadedInner {
    fn installed(&self) -> bool {
        self.handle.is_some()
    }

    fn uninstall(&mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            // A failure here indicates a library issue. Please open an issue on GitHub!
            let mut hooks = INSTALLED_HOOKS
                .write()
                .expect("Unable to obtain write lock");

            let status = unsafe { UnhookWinEvent(handle) };
            match status.as_bool() {
                true => {
                    hooks.remove(&handle.0);

                    trace!(?handle, "uninstalled hook");

                    Ok(())
                }
                false => Err(Error::Uninstallation),
            }
        } else {
            Err(Error::AlreadyUninstalled)
        }
    }
}

impl Drop for UnthreadedInner {
    fn drop(&mut self) {
        if self.installed() {
            self.uninstall().unwrap();
        }
    }
}

pub struct ThreadedInner {
    unthreaded: UnthreadedInner,
    thread_pool: Arc<ThreadPool>,
    thread_pool_tid: u32,
}

impl ThreadedInner {
    pub fn new(config: Config, handler: Box<dyn EventHandler>) -> Result<Self> {
        let thread_name = config
            .dedicated_thread_name
            .clone()
            // A failure here indicates a library issue. Please open an issue on GitHub!
            .expect("Expected a dedicated_thread_name when allocating ThreadedInner");

        let thread_pool = Arc::new(
            ThreadPoolBuilder::new()
                .thread_name(move |i| format!("{thread_name}{i}"))
                .num_threads(1)
                .build()?,
        );

        trace!(?thread_pool, "created thread_pool");

        // obtain the thread pool thread_id
        let thread_pool_tid = thread_pool.install(|| unsafe { GetCurrentThreadId() });

        // create a forwarding handler that invokes on the thread_pool
        let captured_thread_pool = thread_pool.clone();
        let threaded_handler = Box::new(
            move |e: Event, h: WindowHandle, obj: i32, child: i32, thread: u32, time: u32| {
                captured_thread_pool.install(|| {
                    let callback = handler.as_ref();

                    callback(e, h, obj, child, thread, time);
                });
            },
        );

        // ensure the actual hook is installed within the thread_pool
        let unthreaded = thread_pool.install(|| UnthreadedInner::new(config, threaded_handler))?;

        trace!("created UnthreadedInner child for ThreadedInner");

        thread_pool.spawn(|| unsafe {
            run_event_loop();
        });

        trace!("spawned event_loop on thread_pool");

        Ok(Self {
            unthreaded,
            thread_pool,
            thread_pool_tid,
        })
    }
}

impl WinEventHookInner for ThreadedInner {
    fn installed(&self) -> bool {
        self.unthreaded.installed()
    }

    fn uninstall(&mut self) -> Result<()> {
        if self.installed() {
            // stop the event loop
            unsafe { PostThreadMessageW(self.thread_pool_tid, WM_QUIT, WPARAM(0), LPARAM(0)) }
                .ok()?;

            // uninstall the event hook, and return the result
            self.thread_pool.install(|| self.unthreaded.uninstall())
        } else {
            Err(Error::AlreadyUninstalled)
        }
    }
}

impl Drop for ThreadedInner {
    fn drop(&mut self) {
        if self.installed() {
            self.uninstall().unwrap();
        }
    }
}

/// This represents the primitive inner type of [`HWINEVENTHOOK`].
type EventHookId = isize;

lazy_static! {
    /// Storage for hooks that need to be invoked by `__on_win_event_hook_event`.
    static ref INSTALLED_HOOKS: RwLock<HashMap<EventHookId, Weak<(Box<dyn EventHandler>, Option<Vec<Event>>)>>> =
        RwLock::new(HashMap::new());
}

/// System-exposed springboard for raising `win_event_hook` [`EventHandler`] callbacks.
extern "system" fn __on_win_event_hook_event(
    event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: WindowHandle,
    id_object: i32,
    id_child: i32,
    id_event_thread: u32,
    event_time: u32,
) {
    // A failure here indicates a library bug! Please open an issue on GitHub!
    let event =
        Event::try_from(event).expect(&format!("Unable to identify event with value: '{}'", event));
    let hooks = INSTALLED_HOOKS.read().expect("Unable to obtain read lock");
    let event_data = hooks.get(&event_hook.0).expect(&format!(
        "Unable to obtain hook with id: '{}'",
        event_hook.0
    ));

    debug!(
        ?event_hook,
        ?event,
        ?hwnd,
        ?id_object,
        ?id_child,
        ?id_event_thread,
        ?event_time,
        "got event"
    );

    if let Some(event_data) = event_data.upgrade() {
        trace!("got ref to event_data");

        let event_handler = &event_data.0;
        let event_filter = &event_data.1;

        trace!(?event_filter, "filter");

        match event_filter {
            // if we have an event filter only call the handler
            // if the given filter contains our event
            Some(event_filter) => {
                if event_filter.contains(&event) {
                    event_handler(
                        event,
                        hwnd,
                        id_object,
                        id_child,
                        id_event_thread,
                        event_time,
                    );
                }
            }
            // if we have no event filter always call the handler
            None => {
                event_handler(
                    event,
                    hwnd,
                    id_object,
                    id_child,
                    id_event_thread,
                    event_time,
                );
            }
        }
    } else {
        // it's theoretically possible for this to occur for os buffered events after we've uninstalled.
        // As a result, this is implemented as a warning rather than panic.
        warn!("Unable to find event handler with id: '{:?}'", event_hook.0);
    }
}
