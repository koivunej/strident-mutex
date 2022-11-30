//! # `strident-mutex`
//!
//! A easy-to-use `std::sync::{Mutex,RwLock}` tool for development time understanding of the system
//! lock usage by panicking whenever the `{Mutex,RwLock}` are locked from async context.
//!
//! Aim is to keep this crate quite simple, allowing for quick prototyping of other things, such as
//! capturing long holding of in async context.
//!
//! Idea is that the user will have a local git checkout and modify the crate as need be.
#![warn(rust_2018_idioms)]

pub mod sync;

#[cfg(feature = "hide_test_panic")]
static HOOK_LOCK: once_cell::sync::OnceCell<std::sync::Mutex<()>> =
    once_cell::sync::OnceCell::new();

/// Panicks if [`is_within_tokio`] returns true.
#[track_caller]
pub fn panic_if_within_tokio(op: &str) {
    if is_within_tokio() {
        panic!("{op} called within an async context");
    }
}

/// Checks to see if `block_on` panics or not, which is quite expensive.
///
/// Using this outside local development is not recommended. Additionally, a temporary global panic
/// hook is switched while holding a crate specific static (non-screaming) mutex.
pub fn is_within_tokio() -> bool {
    match tokio::runtime::Handle::try_current().ok() {
        Some(handle) => {
            let message = {
                let _g = hide_panic();
                // Safety: block_on might trace, but it will panic before actually touching the
                // runtime
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    handle.block_on(async move {
                        "not panicking means we are outside of async context, but just wasted cycles"
                    })
                }))
            };

            message.is_err()
        }
        None => false,
    }
}

pub(crate) fn hide_panic<'a>() -> Option<PanicHideGuard<'a>> {
    #[cfg(feature = "hide_test_panic")]
    {
        let hook = Box::new(|_: &std::panic::PanicInfo<'_>| {});

        let guard = HOOK_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();
        let old = std::panic::take_hook();
        std::panic::set_hook(hook);

        Some(PanicHideGuard(guard, Some(old)))
    }
    #[cfg(not(feature = "hide_test_panic"))]
    None
}

pub(crate) struct PanicHideGuard<'a>(
    std::sync::MutexGuard<'a, ()>,
    Option<Box<dyn Fn(&std::panic::PanicInfo<'_>) + Send + Sync>>,
);

impl<'a> Drop for PanicHideGuard<'a> {
    fn drop(&mut self) {
        let old = self.1.take().unwrap();
        std::panic::set_hook(old);
    }
}
