use rayon::ThreadPoolBuildError;
use thiserror::Error;

use crate::config::Config;

/// `win_event_hook` library error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Indicates an event with a given id is not known.
    #[error("No known event '{0}'")]
    InvalidEvent(u32),
    /// Indicates an event with a given id falls outside the configured range.
    #[error("Event '{event}' falls outside valid range [{min}, {max}]")]
    InvalidRangedEvent { event: u32, min: u32, max: u32 },
    /// Indicates a config instance was determined to be invalid.
    #[error("Config '{0:?}' is not valid")]
    InvalidConfig(Config),
    /// Indicates an installation failure.
    /// See [Microsoft Documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook#return-value)
    /// for more information.
    #[error("Failed to install WinEventHook")]
    Installation,
    /// Indicates an installation failure due to an underlying threadpool issue.
    #[error("Failed to allocate threadpool")]
    Threadpool(#[from] ThreadPoolBuildError),
    /// Indicates an uninstallation failure.
    /// See [Microsoft Documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwinevent#return-value)
    /// for more information.
    #[error("Failed to uninstall WinEventHook")]
    Uninstallation,
    /// Indicates an uninstallation failure due to an underlying event loop issue.
    #[error("Failed to terminate eventloop")]
    EventLoop(#[from] windows::core::Error),
    /// Indicates an uninstallation failure due to the hook already being uninstalled.
    #[error("Failed to uninstall WinEventHook, already uninstalled")]
    AlreadyUninstalled,
}

/// `win_event_hook` library result type.
pub type Result<T> = std::result::Result<T, Error>;
