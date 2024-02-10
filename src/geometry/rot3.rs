use cxx::UniquePtr;
use nalgebra::{Matrix3, Quaternion, UnitQuaternion};

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

pub struct Rot3Ref<'a> {
    pub(super) inner: &'a ::sys::Rot3,
}

impl<'a> From<Rot3Ref<'a>> for ::nalgebra::UnitQuaternion<f64> {
    fn from(value: Rot3Ref<'a>) -> Self {
        let mut dst = [0.0; 9];
        ::sys::rot3_to_raw(value.inner, &mut dst);

        let [m11, m12, m13, m21, m22, m23, m31, m32, m33] = dst;
        ::nalgebra::UnitQuaternion::from_matrix(&Matrix3::new(
            m11, m12, m13, // row 1
            m21, m22, m23, // row 2
            m31, m32, m33, // row 3
        ))
    }
}
