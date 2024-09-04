use crate::{events::Event, handles::WindowHandle};

/// Signature of the Event Hook callback function.
pub trait EventHandler: Fn(Event, WindowHandle, i32, i32, u32, u32) + Sync + Send {}

impl<T> EventHandler for T where T: Fn(Event, WindowHandle, i32, i32, u32, u32) + Sync + Send {}
