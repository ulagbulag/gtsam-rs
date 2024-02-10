use cxx::UniquePtr;

use crate::{
    geometry::pose3::Pose3, inference::symbol::Symbol, linear::noise_model::DiagonalNoiseModel,
};

pub struct NonlinearFactorGraph {
    inner: UniquePtr<::sys::NonlinearFactorGraph>,
}

impl Default for NonlinearFactorGraph {
    fn default() -> Self {
        Self {
            inner: ::sys::default_nonlinear_factor_graph(),
        }
    }
}

impl NonlinearFactorGraph {
    pub fn add_prior(&mut self, symbol: &Symbol, prior: &Pose3, model: &DiagonalNoiseModel) {
        ::sys::nonlinear_factor_graph_add_prior(
            self.inner.pin_mut(),
            symbol.key(),
            &prior.inner,
            &model.to_base_model().inner,
        )
    }
}
