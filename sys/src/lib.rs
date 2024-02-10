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

        fn point3_to_raw(src: &Point3, dst: &mut [f64]);
    }

    unsafe extern "C++" {
        include!("geometry/pose3.h");

        type Pose3;

        fn default_pose3() -> UniquePtr<Pose3>;

        fn new_pose3(rotation: &Rot3, point: &Point3) -> UniquePtr<Pose3>;

        fn pose3_rotation(pose: &Pose3) -> &Rot3;

        fn pose3_translation(pose: &Pose3) -> &Point3;
    }

    unsafe extern "C++" {
        include!("geometry/rot3.h");

        type Rot3;

        fn default_rot3() -> UniquePtr<Rot3>;

        fn from_rot3_quaternion(w: f64, x: f64, y: f64, z: f64) -> UniquePtr<Rot3>;

        fn rot3_to_raw(src: &Rot3, dst: &mut [f64]);
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
        include!("nonlinear/levenberg_marquardt_optimizer.h");

        type LevenbergMarquardtOptimizer;

        fn new_levenberg_marquardt_optimizer(
            graph: &NonlinearFactorGraph,
            initial_values: &Values,
            params: &LevenbergMarquardtParams,
        ) -> UniquePtr<LevenbergMarquardtOptimizer>;

        fn optimizeSafely(self: Pin<&mut Self>) -> &Values;
    }

    unsafe extern "C++" {
        include!("nonlinear/levenberg_marquardt_params.h");

        type LevenbergMarquardtParams;

        fn default_levenberg_marquardt_params() -> UniquePtr<LevenbergMarquardtParams>;

        fn levenberg_marquardt_params_set_max_iterations(
            params: Pin<&mut LevenbergMarquardtParams>,
            value: u32,
        );
    }

    unsafe extern "C++" {
        include!("nonlinear/nonlinear_factor_graph.h");

        type NonlinearFactorGraph;

        fn default_nonlinear_factor_graph() -> UniquePtr<NonlinearFactorGraph>;

        fn nonlinear_factor_graph_add_between_factor_pose3(
            graph: Pin<&mut NonlinearFactorGraph>,
            key1: u64,
            key2: u64,
            measured: &Pose3,
            model: &SharedPtr<BaseNoiseModel>,
        );

        fn nonlinear_factor_graph_add_prior_factor_pose3(
            graph: Pin<&mut NonlinearFactorGraph>,
            key: u64,
            prior: &Pose3,
            model: &SharedPtr<BaseNoiseModel>,
        );
    }

    unsafe extern "C++" {
        include!("nonlinear/values.h");

        type Values;

        fn default_values() -> UniquePtr<Values>;

        fn values_at_pose3(values: &Values, key: u64) -> &Pose3;

        fn values_exists(values: &Values, key: u64) -> bool;

        fn values_insert_pose3(values: Pin<&mut Values>, key: u64, value: &Pose3);
    }
}
