use cxx::UniquePtr;
use nalgebra::{Quaternion, UnitQuaternion};

pub struct Rot3 {
    pub(crate) inner: UniquePtr<::sys::Rot3>,
}

impl Default for Rot3 {
    fn default() -> Self {
        Self {
            inner: ::sys::default_rot3(),
        }
    }
}

impl From<Quaternion<f64>> for Rot3 {
    fn from(value: Quaternion<f64>) -> Self {
        let quaternion = value.as_vector();

        Self {
            inner: ::sys::from_rot3_quaternion(
                quaternion.w,
                quaternion.x,
                quaternion.y,
                quaternion.z,
            ),
        }
    }
}

impl From<UnitQuaternion<f64>> for Rot3 {
    fn from(value: UnitQuaternion<f64>) -> Self {
        let quaternion = value.as_vector();

        Self {
            inner: ::sys::from_rot3_quaternion(
                quaternion.w,
                quaternion.x,
                quaternion.y,
                quaternion.z,
            ),
        }
    }
}
