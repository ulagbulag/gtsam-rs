use cxx::UniquePtr;

type Inner = ::sys::geometry::pose3::Pose3;

pub struct Pose3 {
    inner: UniquePtr<Inner>,
}

impl Default for Pose3 {
    fn default() -> Self {
        Self {
            inner: ::sys::geometry::pose3::new_pose3(),
        }
    }
}
