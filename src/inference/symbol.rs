use cxx::UniquePtr;
use gtsam_sys::Key;

pub struct Symbol {
    inner: UniquePtr<::sys::Symbol>,
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            inner: ::sys::default_symbol(),
        }
    }
}

impl Symbol {
    pub fn new(index: u64) -> Self {
        Self {
            inner: ::sys::new_symbol(index),
        }
    }

    pub(crate) fn key(&self) -> Key {
        self.inner.key()
    }
}
