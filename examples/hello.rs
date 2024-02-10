use gtsam::{
    inference::symbol::Symbol, linear::noise_model::DiagonalNoiseModel,
    nonlinear::nonlinear_factor_graph::NonlinearFactorGraph,
};
use nalgebra::{Isometry3, Vector3, Vector6};

fn main() {
    let mut graph = NonlinearFactorGraph::default();

    let prior_noise =
        DiagonalNoiseModel::from_sigmas(Vector6::new(1e-6, 1e-6, 1e-6, 1e-4, 1e-4, 1e-4));
    graph.add_prior(
        &Symbol::new(1),
        &Isometry3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)).into(),
        &prior_noise,
    );
}
