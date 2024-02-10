use cxx::UniquePtr;

use crate::{
    geometry::pose3::Pose3, inference::key::IntoKey, linear::noise_model::DiagonalNoiseModel,
};

pub struct NonlinearFactorGraph {
    pub(super) inner: UniquePtr<::sys::NonlinearFactorGraph>,
}

impl Default for NonlinearFactorGraph {
    fn default() -> Self {
        Self {
            inner: ::sys::default_nonlinear_factor_graph(),
        }
    }
}

impl NonlinearFactorGraph {
    pub fn add_between_factor_pose3(
        &mut self,
        from: impl IntoKey,
        to: impl IntoKey,
        measured: &Pose3,
        model: &DiagonalNoiseModel,
    ) {
        ::sys::nonlinear_factor_graph_add_between_factor_pose3(
            self.inner.pin_mut(),
            from.into_key(),
            to.into_key(),
            &measured.inner,
            &model.to_base_model().inner,
        )
    }

    pub fn add_prior_factor_pose3(
        &mut self,
        symbol: impl IntoKey,
        prior: &Pose3,
        model: &DiagonalNoiseModel,
    ) {
        ::sys::nonlinear_factor_graph_add_prior_factor_pose3(
            self.inner.pin_mut(),
            symbol.into_key(),
            &prior.inner,
            &model.to_base_model().inner,
        )
    }
}
