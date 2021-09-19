//! TODO(BSFishy): document this

/// TODO(BSFishy): document this
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    /// TODO(BSFishy): document this
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    /// TODO(BSFishy): document this
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}
