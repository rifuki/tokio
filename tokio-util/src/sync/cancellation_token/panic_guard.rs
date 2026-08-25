use crate::sync::CancellationToken;

/// A wrapper for a cancellation token which cancels it on drop, but only if the
/// current thread is panicking. It is created using the [`panic_guard`] method
/// on the [`CancellationToken`].
///
/// This is a conditional counterpart to [`DropGuard`], which cancels
/// unconditionally.
///
/// # Caveats
///
/// The guard cancels the token whenever it is dropped while the current thread
/// is unwinding, which is not the same as "the code this guard was protecting
/// panicked". [`std::thread::panicking`] stays `true` for the entire unwind, so
/// a `PanicGuard` dropped inside a [`Drop`] implementation that itself runs as
/// part of an unrelated panic will also cancel the token.
///
/// When the binary is built with `panic = "abort"` there is no unwinding, so a
/// `PanicGuard` never cancels the token.
///
/// [`panic_guard`]: CancellationToken::panic_guard
/// [`DropGuard`]: super::DropGuard
#[derive(Debug)]
pub struct PanicGuard {
    pub(super) inner: Option<CancellationToken>,
}

impl PanicGuard {
    /// Returns a reference to the cancellation token wrapped by this guard.
    pub fn token(&self) -> &CancellationToken {
        self.inner
            .as_ref()
            .expect("`inner` can only be None in a destructor")
    }

    /// Returns the stored cancellation token and removes this guard instance
    /// (i.e. it will no longer cancel the token). Other guards for this token
    /// are not affected.
    pub fn disarm(mut self) -> CancellationToken {
        self.inner
            .take()
            .expect("`inner` can be only None in a destructor")
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            if let Some(inner) = &self.inner {
                inner.cancel();
            }
        }
    }
}
