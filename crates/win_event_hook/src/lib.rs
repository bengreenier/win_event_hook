pub use config::Config;
use errors::{Error, Result};
pub use handler::EventHandler;
use hook::{ThreadedInner, UnthreadedInner, WinEventHookInner};
use tracing::trace;

pub mod config;
pub mod errors;
mod event_loop;
pub mod events;
pub mod flags;
pub mod handler;
mod hook;

/// A Windows Event Hook, managed using the
/// [SetWinEventHook](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook)
/// and
/// [UnhookWinEvent](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwinevent)
/// Windows API functions.
pub struct WinEventHook {
    inner: Box<dyn WinEventHookInner>,
}

impl WinEventHook {
    /// Determines if the hook is currently installed.
    pub fn installed(&self) -> bool {
        self.inner.installed()
    }

    /// Installs a hook, using a given [`Config`] and [`EventHandler`] function.
    ///
    /// Note: [`Config`] can be created using the builder pattern, with [`Config::builder`].
    pub fn install<F: EventHandler + 'static>(config: Config, handler: F) -> Result<Self> {
        trace!(?config, "validating config");

        if !config.is_valid() {
            return Err(Error::InvalidConfig(config));
        }

        trace!("config valid, attempting to install hook");

        Ok(Self {
            inner: match config.dedicated_thread_name.is_none() {
                true => Box::new(UnthreadedInner::new(config, Box::new(handler))?),
                false => Box::new(ThreadedInner::new(config, Box::new(handler))?),
            },
        })
    }

    /// Uninstalls a hook, if it is not currently installed.
    pub fn uninstall(&mut self) -> Result<()> {
        self.inner.uninstall()
    }
}

#[cfg(test)]
mod tests {

    use tracing::info;
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    use super::{
        events::{Event, NamedEvent},
        Config, WinEventHook,
    };

    #[test]
    fn can_install_threaded() {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();

        let cfg = Config::builder()
            .with_event(Event::Named(NamedEvent::ObjectShow))
            .with_event(Event::Named(NamedEvent::ObjectNameChange))
            .with_event(Event::Named(NamedEvent::ObjectHide))
            .with_dedicated_thread()
            .finish();

        let mut hook =
            WinEventHook::install(cfg, |ev, _, _, _, _, _| info!(?ev, "got event")).unwrap();

        hook.uninstall().unwrap();
    }
}