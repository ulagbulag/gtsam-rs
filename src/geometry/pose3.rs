use cxx::UniquePtr;
use nalgebra::{Isometry3, UnitQuaternion, Vector3};

use super::{point3::Point3, rot3::Rot3};

pub struct Pose3 {
    pub(crate) inner: UniquePtr<::sys::Pose3>,
}

impl Default for Pose3 {
    fn default() -> Self {
        Self {
            inner: ::sys::default_pose3(),
        }
    }
}

impl From<Isometry3<f64>> for Pose3 {
    fn from(value: Isometry3<f64>) -> Self {
        Self::from_parts(value.translation.into(), value.rotation.into())
    }
}

impl Pose3 {
    pub fn new(translation: Vector3<f64>, axisangle: Vector3<f64>) -> Self {
        Self::from_parts(
            translation.into(),
            UnitQuaternion::from_scaled_axis(axisangle).into(),
        )
    }

    pub fn from_parts(point: Point3, rotation: Rot3) -> Self {
        Self {
            inner: ::sys::new_pose3(&rotation.inner, &point.inner),
        }
    }
}
