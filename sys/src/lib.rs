pub use self::ffi::*;

// Common types
pub type Key = u64;

#[cxx::bridge(namespace = "gtsam")]
mod ffi {
    unsafe extern "C++" {
        include!("geometry/point3.h");

        type Point3;

        fn default_point3() -> UniquePtr<Point3>;

        fn new_point3(x: f64, y: f64, z: f64) -> UniquePtr<Point3>;
    }

    unsafe extern "C++" {
        include!("geometry/pose3.h");

        type Pose3;

        fn default_pose3() -> UniquePtr<Pose3>;

        fn new_pose3(rotation: &Rot3, point: &Point3) -> UniquePtr<Pose3>;
    }

    unsafe extern "C++" {
        include!("geometry/rot3.h");

        type Rot3;

        fn default_rot3() -> UniquePtr<Rot3>;

        fn from_rot3_quaternion(w: f64, x: f64, y: f64, z: f64) -> UniquePtr<Rot3>;
    }

    unsafe extern "C++" {
        include!("inference/symbol.h");

        type Symbol;

        fn default_symbol() -> UniquePtr<Symbol>;

        fn new_symbol(index: u64) -> UniquePtr<Symbol>;

        fn key(&self) -> u64;
    }

    unsafe extern "C++" {
        include!("linear/noise_model.h");

        type BaseNoiseModel;
    }

    unsafe extern "C++" {
        include!("linear/noise_model.h");

        type DiagonalNoiseModel;

        fn from_diagonal_noise_model_sigmas(sigmas: &mut [f64]) -> SharedPtr<DiagonalNoiseModel>;

        fn cast_diagonal_noise_model_to_base_noise_model(
            a: &SharedPtr<DiagonalNoiseModel>,
        ) -> SharedPtr<BaseNoiseModel>;
    }

    unsafe extern "C++" {
        include!("nonlinear/nonlinear_factor.h");

        type NonlinearFactor;
    }

    unsafe extern "C++" {
        include!("nonlinear/nonlinear_factor_graph.h");

        type NonlinearFactorGraph;

        fn default_nonlinear_factor_graph() -> UniquePtr<NonlinearFactorGraph>;

        fn nonlinear_factor_graph_add_prior(
            graph: Pin<&mut NonlinearFactorGraph>,
            key: u64,
            prior: &Pose3,
            model: &SharedPtr<BaseNoiseModel>,
        );
    }
}
