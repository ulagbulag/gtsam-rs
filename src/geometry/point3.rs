use cxx::UniquePtr;

pub struct Point3 {
    pub(crate) inner: UniquePtr<::sys::Point3>,
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            inner: ::sys::default_point3(),
        }
    }
}

impl From<::nalgebra::Point3<f64>> for Point3 {
    fn from(value: ::nalgebra::Point3<f64>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<::nalgebra::Translation3<f64>> for Point3 {
    fn from(value: ::nalgebra::Translation3<f64>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<::nalgebra::Vector3<f64>> for Point3 {
    fn from(value: ::nalgebra::Vector3<f64>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: ::sys::new_point3(x, y, z),
        }
    }
}
