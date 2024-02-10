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

impl From<::nalgebra::Vector3<f64>> for Point3 {
    fn from(value: ::nalgebra::Vector3<f64>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<::nalgebra::Translation3<f64>> for Point3 {
    fn from(value: ::nalgebra::Translation3<f64>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<::nalgebra::Point3<f64>> for Point3 {
    fn from(value: ::nalgebra::Point3<f64>) -> Self {
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

pub struct Point3Ref<'a> {
    pub(super) inner: &'a ::sys::Point3,
}

impl<'a> From<Point3Ref<'a>> for ::nalgebra::Vector3<f64> {
    fn from(value: Point3Ref<'a>) -> Self {
        let mut dst = [0.0; 3];
        ::sys::point3_to_raw(value.inner, &mut dst);

        let [x, y, z] = dst;
        ::nalgebra::Vector3::new(x, y, z)
    }
}

impl<'a> From<Point3Ref<'a>> for ::nalgebra::Translation3<f64> {
    fn from(value: Point3Ref<'a>) -> Self {
        ::nalgebra::Vector3::from(value).into()
    }
}

impl<'a> From<Point3Ref<'a>> for ::nalgebra::Point3<f64> {
    fn from(value: Point3Ref<'a>) -> Self {
        ::nalgebra::Vector3::from(value).into()
    }
}
