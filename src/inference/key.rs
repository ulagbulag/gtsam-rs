use gtsam_sys::Key;

use crate::inference::symbol::Symbol;

pub trait IntoKey
where
    Self: ToKeySealed,
{
    fn into_key(self) -> Key;
}

impl<T> IntoKey for &T
where
    T: IntoKey,
{
    fn into_key(self) -> Key {
        <T as IntoKey>::into_key(*self)
    }
}

impl IntoKey for u64 {
    fn into_key(self) -> Key {
        self
    }
}

impl IntoKey for &Symbol {
    fn into_key(&self) -> Key {
        self.key()
    }
}
#[doc(hidden)]
mod sealed {
    use super::*;

    pub trait ToKeySealed {}

    impl<T> ToKeySealed for &T where T: ToKeySealed {}

    impl ToKeySealed for u64 {}

    impl ToKeySealed for Symbol {}
}
