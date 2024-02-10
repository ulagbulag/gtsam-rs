use std::f64;

use gtsam::{
    inference::symbol::Symbol,
    linear::noise_model::DiagonalNoiseModel,
    nonlinear::{
        levenberg_marquardt_optimizer::LevenbergMarquardtOptimizer,
        levenberg_marquardt_params::LevenbergMarquardtParams,
        nonlinear_factor_graph::NonlinearFactorGraph, values::Values,
    },
};
use nalgebra::{Isometry3, Vector3, Vector6};

fn main() {
    let mut graph = NonlinearFactorGraph::default();

    let prior_noise =
        DiagonalNoiseModel::from_sigmas(Vector6::new(1e-6, 1e-6, 1e-6, 1e-4, 1e-4, 1e-4));
    graph.add_prior_factor_pose3(
        &Symbol::new(1),
        &Isometry3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)).into(),
        &prior_noise,
    );

    // Add odometry factors
    let dist = 2.0;
    let cov_noise = DiagonalNoiseModel::from_sigmas(Vector6::new(0.5, 0.5, 0.5, 0.1, 0.1, 0.1));
    graph.add_between_factor_pose3(
        1,
        2,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -f64::consts::PI / 2.0, 0.0),
        )
        .into(),
        &cov_noise,
    );
    graph.add_between_factor_pose3(
        2,
        3,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -f64::consts::PI / 2.0, 0.0),
        )
        .into(),
        &cov_noise,
    );
    graph.add_between_factor_pose3(
        3,
        4,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -f64::consts::PI / 2.0, 0.0),
        )
        .into(),
        &cov_noise,
    );
    graph.add_between_factor_pose3(
        4,
        5,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -f64::consts::PI / 2.0, 0.0),
        )
        .into(),
        &cov_noise,
    );

    // Add loop closure factors
    let loop_noise = cov_noise;
    graph.add_between_factor_pose3(
        5,
        2,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -f64::consts::PI / 2.0, 0.0),
        )
        .into(),
        &loop_noise,
    );

    let mut initials = Values::default();
    initials.insert_pose3(
        1,
        &Isometry3::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, -0.0 * f64::consts::PI / 2.0, 0.0),
        )
        .into(),
    );
    initials.insert_pose3(
        2,
        &Isometry3::new(
            Vector3::new(dist, 0.0, 0.0),
            Vector3::new(0.0, -1.0 * f64::consts::PI / 2.0, 0.0),
        )
        .into(),
    );
    initials.insert_pose3(
        3,
        &Isometry3::new(
            Vector3::new(dist, 0.0, dist),
            Vector3::new(0.0, -2.0 * f64::consts::PI / 2.0, 0.0),
        )
        .into(),
    );
    initials.insert_pose3(
        4,
        &Isometry3::new(
            Vector3::new(0.0, 0.0, dist),
            Vector3::new(0.0, -3.0 * f64::consts::PI / 2.0, 0.0),
        )
        .into(),
    );
    initials.insert_pose3(
        5,
        &Isometry3::new(
            Vector3::new(0.0, 0.0, dist),
            Vector3::new(0.0, -4.0 * f64::consts::PI / 2.0, 0.0),
        )
        .into(),
    );

    let mut opt_param = LevenbergMarquardtParams::default();
    opt_param.set_max_iterations(200);

    let mut opt = LevenbergMarquardtOptimizer::new(&graph, &initials, &opt_param);

    let result = opt.optimize_safely();
    for i in 1..=5 {
        let pose: Isometry3<f64> = result.get_pose3(i).unwrap().into();
        println!("[{i}] {pose}");
    }
}
