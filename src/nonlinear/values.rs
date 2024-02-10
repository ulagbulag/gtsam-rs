use cxx::UniquePtr;

use crate::{
    geometry::pose3::{Pose3, Pose3Ref},
    inference::symbol::Symbol,
};

pub struct Values {
    pub(super) inner: UniquePtr<::sys::Values>,
}

impl Default for Values {
    fn default() -> Self {
        Self {
            inner: ::sys::default_values(),
        }
    }
}

impl Values {
    pub fn insert_pose3(&mut self, key: &Symbol, value: &Pose3) {
        ::sys::values_insert_pose3(self.inner.pin_mut(), key.key(), &value.inner)
    }
}

pub struct ValuesRef<'a> {
    pub(super) inner: &'a ::sys::Values,
}

impl<'a> ValuesRef<'a> {
    pub fn get_pose3(&self, key: &Symbol) -> Option<Pose3Ref> {
        let key = key.key();

        if ::sys::values_exists(self.inner, key) {
            Some(Pose3Ref {
                inner: ::sys::values_at_pose3(self.inner, key),
            })
        } else {
            None
        }
    }
}
