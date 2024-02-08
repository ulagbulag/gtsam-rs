pub use self::ffi::*;

#[cxx::bridge(namespace = "gtsam")]
mod ffi {
    unsafe extern "C++" {
        include!("geometry/pose3.h");

        type Pose3;

        fn new_pose3() -> UniquePtr<Pose3>;
    }
}
