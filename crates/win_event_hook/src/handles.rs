use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

/// Built-in [`PlatformHandle`] implementations for commonly used windows handle types.
pub mod builtins {
    use std::{ffi::c_void, hash::Hash};

    use windows::Win32::{
        Foundation::{HMODULE, HWND},
        UI::Accessibility::HWINEVENTHOOK,
    };

    use super::PlatformHandle;

    /// Re-exported [`HWINEVENTHOOK`].
    pub type OsHandle = HWINEVENTHOOK;

    impl PlatformHandle for OsHandle {
        type Primitive = *mut c_void;

        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // Cast the reference to a raw pointer and hash the address
            let ptr = self.0 as *const c_void as usize;

            ptr.hash(state);
        }
    }

    /// Re-exported [`windows::Win32::Foundation::HMODULE`].
    pub type ModuleHandle = HMODULE;

    impl PlatformHandle for ModuleHandle {
        type Primitive = *mut c_void;

        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // Cast the reference to a raw pointer and hash the address
            let ptr = self.0 as *const c_void as usize;

            ptr.hash(state);
        }
    }

    /// Re-exported [`windows::Win32::Foundation::HWND`].
    pub type WindowHandle = HWND;

    impl PlatformHandle for WindowHandle {
        type Primitive = *mut c_void;

        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // Cast the reference to a raw pointer and hash the address
            let ptr = self.0 as *const c_void as usize;

            ptr.hash(state);
        }
    }
}

/// A [`Handle`] containing `HWINEVENTHOOK` under-the-hood.
pub type OsHandle = OpaqueHandle<builtins::OsHandle>;

/// A [`Handle`] containing `HMODULE` under-the-hood.
pub type ModuleHandle = OpaqueHandle<builtins::ModuleHandle>;

/// A [`Handle`] containing `HWND` under-the-hood.
pub type WindowHandle = OpaqueHandle<builtins::WindowHandle>;

/// Abstraction for platform handles (see `builtins`) that constrain them to align with our requirements.
pub trait PlatformHandle: Debug + Clone + PartialEq + Eq {
    /// The type of the handles underlying value. For instance, `isize` or `*mut c_void`.
    type Primitive: Debug + Clone + PartialEq + Eq;

    // TODO(bengreenier): not sure why contstraining to `Hash` doesn't work, but it doesn't.
    fn hash<H: Hasher>(&self, state: &mut H);
}

/// Abstraction for application handles used throughout this library.
///
/// See [`OpaqueHandle`].
pub trait Handle: Debug + Clone + PartialEq + Eq + Hash {
    type PlatformHandle: PlatformHandle;
}

/// Abstraction for application handles that allow them to meet our safety requirements.
///
/// This is what we store within the `win_event_hook` library, and callers may store within
/// their application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpaqueHandle<T: PlatformHandle>(T);

impl<T> Handle for OpaqueHandle<T>
where
    T: PlatformHandle,
{
    type PlatformHandle = T;
}

unsafe impl<T> Sync for OpaqueHandle<T> where T: PlatformHandle {}
unsafe impl<T> Send for OpaqueHandle<T> where T: PlatformHandle {}

impl<T> From<T> for OpaqueHandle<T>
where
    T: PlatformHandle,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for OpaqueHandle<T>
where
    T: PlatformHandle,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OpaqueHandle<T>
where
    T: PlatformHandle,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Hash for OpaqueHandle<T>
where
    T: PlatformHandle,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Default for OpaqueHandle<T>
where
    T: PlatformHandle + Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}
