pub use self::ffi::*;

#[cxx::bridge(namespace = "gtsam")]
mod ffi {
    unsafe extern "C++" {
        include!("nonlinear/nonlinear_factor_graph.h");

        type NonlinearFactorGraph;

        fn new_nonlinear_factor_graph() -> UniquePtr<NonlinearFactorGraph>;
    }
}
