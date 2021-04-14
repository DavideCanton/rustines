use std::{ops, marker};

/// Add `Sync` to an arbitrary type.
///
/// This primitive is used to get around the `Sync` requirement in `static`s (even thread local
/// ones! see rust-lang/rust#35035). Due to breaking invariants, creating a value of such type is
/// unsafe, and care must be taken upon usage.
///
/// In general, this should only be used when you know it won't be shared across threads (e.g. the
/// value is stored in a thread local variable).
pub struct Syncify<T>(T);

impl<T> Syncify<T> {
    /// Create a new `Syncify` wrapper.
    ///
    /// # Safety
    ///
    /// This is invariant-breaking and thus unsafe.
    pub unsafe fn new(inner: T) -> Syncify<T> {
        Syncify(inner)
    }
}

impl<T> ops::Deref for Syncify<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

unsafe impl<T> marker::Sync for Syncify<T> {}

/// Declare a thread-local static variable.
///
/// TLS works by copying the initial data on every new thread creation. This allows access to a
/// variable, which is only available for the current thread, meaning that there is no need for
/// synchronization.
///
/// For this reason, in contrast to other `static`s in Rust, this need not thread-safety, which is
/// what this macro "fixes".
#[macro_export]
macro_rules! tls {
    (static $name:ident: $ty:ty = $val:expr) => { tls!(#[] static $name: $ty = $val); };
    (#[$($attr:meta),*] static $name:ident: $ty:ty = $val:expr) => {
        $(#[$attr])*
        #[thread_local]
        static $name: Syncify<$ty> = unsafe { Syncify::new($val) };
    }
}
