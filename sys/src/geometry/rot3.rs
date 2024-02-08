pub use self::ffi::*;

#[cxx::bridge(namespace = "gtsam")]
mod ffi {
    unsafe extern "C++" {
        include!("geometry/rot3.h");

        type Rot3;

        fn new_rot3() -> UniquePtr<Rot3>;
    }
}
