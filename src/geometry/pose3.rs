use cxx::UniquePtr;
use nalgebra::{Isometry3, UnitQuaternion, Vector3};

use super::{
    point3::{Point3, Point3Ref},
    rot3::{Rot3, Rot3Ref},
};

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

pub struct Pose3Ref<'a> {
    pub(crate) inner: &'a ::sys::Pose3,
}

impl<'a> From<Pose3Ref<'a>> for Isometry3<f64> {
    fn from(value: Pose3Ref<'a>) -> Self {
        Isometry3::from_parts(value.translation().into(), value.rotation().into())
    }
}

impl Pose3Ref<'_> {
    pub fn rotation(&self) -> Rot3Ref {
        Rot3Ref {
            inner: ::sys::pose3_rotation(self.inner),
        }
    }

    pub fn translation(&self) -> Point3Ref {
        Point3Ref {
            inner: ::sys::pose3_translation(self.inner),
        }
    }
}
