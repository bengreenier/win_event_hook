use windows::Win32::Foundation::HMODULE;

use crate::events::Event;
use crate::flags::Flags;

/// Re-exported [`windows::Win32::Foundation::HMODULE`].
pub type ModuleHandle = HMODULE;

/// Config for
/// [SetWinEventHook](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Config {
    /// Specifies the event constant for the lowest event value in the range of events that are handled by the hook function. This parameter can be set to EVENT_MIN to indicate the lowest possible event value.
    pub event_min: u32,
    /// Specifies the event constant for the highest event value in the range of events that are handled by the hook function. This parameter can be set to EVENT_MAX to indicate the highest possible event value.
    pub event_max: u32,
    /// Specifies an additional filter that will be used to further limit events within the given range.
    pub event_filter: Option<Vec<Event>>,
    /// Specifies the ID of the process from which the hook function receives events. Specify zero (0) to receive events from all processes on the current desktop.
    pub id_process: u32,
    /// Specifies the ID of the thread from which the hook function receives events. If this parameter is zero, the hook function is associated with all existing threads on the current desktop.
    pub id_thread: u32,
    /// Handle to the DLL that contains the hook function.
    pub module_handle: Option<ModuleHandle>,
    /// Flag values that specify the location of the hook function and of the events to be skipped.
    pub dw_flags: Flags,
    /// Specifies the name (and existence) of a thread that will be used for hook management.
    pub dedicated_thread_name: Option<String>,
}

impl Config {
    /// Returns a new [`ConfigBuilder`] for creating a [`Config`] instance.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Determines if the given config is valid, as defined in
    /// [the Windows API documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook).
    pub fn is_valid(&self) -> bool {
        // Check requirement: event min is a permitted value
        self.event_min >= Event::MIN
            // Check requirement: event max is a permitted value
            && self.event_max <= Event::MAX
            // Check requirement: dw_flags are in a valid arrangement
            && self.dw_flags.is_valid()
            // Check requirement: dw_flags && module_handle alignment 
            // if the WINEVENT_INCONTEXT flag is specified in the dwFlags parameter.
            // If the hook function is not located in a DLL, or if the WINEVENT_OUTOFCONTEXT flag
            // is specified, this parameter is NULL.
            && ((self.dw_flags.contains(Flags::IN_CONTEXT) && self.module_handle.is_some())
                || (self.dw_flags.contains(Flags::OUT_OF_CONTEXT) && self.module_handle.is_none()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            event_min: Event::MIN,
            event_max: Event::MAX,
            event_filter: None,
            id_process: 0,
            id_thread: 0,
            module_handle: None,
            dw_flags: Flags::default(),
            dedicated_thread_name: None,
        }
    }
}

/// A builder for creating new [`Config`] instances.
#[derive(Default)]
pub struct ConfigBuilder {
    inner: Config,
}

impl ConfigBuilder {
    /// Returns a new [`ConfigBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a particular [`Event`] to be captured by the hook.
    ///
    /// Note: Should not be mixed with the `with_event_range` builder method.
    pub fn with_event(self, event: Event) -> Self {
        let mut event_min = self.inner.event_min;
        let mut event_max = self.inner.event_max;
        let id: u32 = event.into();

        if id < event_min || event_min == Event::MIN {
            event_min = id;
        }

        if id > event_max || event_max == Event::MAX {
            event_max = id;
        }

        let mut event_filter = self.inner.event_filter.unwrap_or(Vec::new());

        event_filter.push(event);

        Self {
            inner: Config {
                event_min,
                event_max,
                event_filter: Some(event_filter),
                ..self.inner
            },
        }
    }

    /// Adds a particular set of [`Event`]s to be captured by the hook.
    ///
    /// Note: Should not be mixed with the `with_event_range` builder method.
    pub fn with_events<T: Into<Vec<Event>>>(self, events: T) -> Self {
        let mut event_min = self.inner.event_min;
        let mut event_max = self.inner.event_max;
        let mut event_filter = self.inner.event_filter.unwrap_or(Vec::new());

        for event in events.into() {
            let id: u32 = event.into();

            if id < event_min || event_min == Event::MIN {
                event_min = id;
            }

            if id > event_max || event_max == Event::MAX {
                event_max = id;
            }

            event_filter.push(event);
        }

        Self {
            inner: Config {
                event_min,
                event_max,
                event_filter: Some(event_filter),
                ..self.inner
            },
        }
    }

    /// Adds a particular range of [`Event`] ids to be captured by the hook.
    ///
    /// Note: Should not be mixed with `with_event`, `with_events` builder methods.
    pub fn with_event_range(self, min: u32, max: u32) -> Self {
        let event_min = if self.inner.event_min > min {
            min
        } else {
            self.inner.event_min
        };

        let event_max = if self.inner.event_max < max {
            max
        } else {
            self.inner.event_max
        };

        Self {
            inner: Config {
                event_min,
                event_max,
                ..self.inner
            },
        }
    }

    /// Sets a particular [`ModuleHandle`] which contains the system hook function to invoke.
    ///
    /// Note: This is for advanced use cases; while it's technically supported, you probably don't want this.
    /// To that end, if you're using this method and looking to improve the ergonomics, please open an issue on GitHub!
    pub fn with_module_context(self, module_handle: ModuleHandle) -> Self {
        // ensure the IN_CONTEXT is removed from the existing flags
        let mut dw_flags = self.inner.dw_flags;
        dw_flags.remove(Flags::IN_CONTEXT);

        // then add the out of context flag
        let dw_flags = dw_flags.union(Flags::OUT_OF_CONTEXT);

        Self {
            inner: Config {
                dw_flags,
                module_handle: Some(module_handle),
                ..self.inner
            },
        }
    }

    /// Sets a particular system process id to scope captured events.
    pub fn with_process_id(self, process_id: u32) -> Self {
        Self {
            inner: Config {
                id_process: process_id,
                ..self.inner
            },
        }
    }

    /// Sets a particular system thread id to scope captured events.
    pub fn with_thread_id(self, thread_id: u32) -> Self {
        Self {
            inner: Config {
                id_thread: thread_id,
                ..self.inner
            },
        }
    }

    /// Configures the hook to use a dedicated thread, managed by this library.
    ///
    /// Note: Since event hooks require an event loop to use, this can be helpful to use when your
    /// application does not have an event loop, as one will be created for you on the dedicated thread.
    pub fn with_dedicated_thread(self) -> Self {
        Self {
            inner: Config {
                dedicated_thread_name: Some("WinEventHookThread".to_string()),
                ..self.inner
            },
        }
    }

    /// Configures the hook to use a dedicated thread, with a given name, managed by this library.
    ///
    /// See [`Self::with_dedicated_thread`] for more information.
    pub fn with_dedicated_thread_name(self, name: &str) -> Self {
        Self {
            inner: Config {
                dedicated_thread_name: Some(name.to_string()),
                ..self.inner
            },
        }
    }

    /// Configures the hook to ignore events raised by the current process id.
    pub fn skip_own_process(self) -> Self {
        Self {
            inner: Config {
                dw_flags: self.inner.dw_flags.union(Flags::SKIP_OWN_PROCESS),
                ..self.inner
            },
        }
    }

    /// Configures the hook to ignore events raised by the current thread.
    ///
    /// Note: cannot be used with [`Self::skip_own_process`] as that is redudant.
    /// Use [`Self::skip_own_process`] instead.
    pub fn skip_own_thread(self) -> Self {
        Self {
            inner: Config {
                dw_flags: self.inner.dw_flags.union(Flags::SKIP_OWN_THREAD),
                ..self.inner
            },
        }
    }

    /// Finish the builder, returning a new [`Config`] instance.
    pub fn finish(self) -> Config {
        self.inner
    }
}
