use cxx::SharedPtr;
use nalgebra::SVector;

pub struct BaseNoiseModel {
    pub(crate) inner: SharedPtr<::sys::BaseNoiseModel>,
}

pub struct DiagonalNoiseModel {
    pub(crate) inner: SharedPtr<::sys::DiagonalNoiseModel>,
}

impl DiagonalNoiseModel {
    pub fn from_sigmas<const D: usize>(mut sigmas: SVector<f64, D>) -> Self {
        Self {
            inner: ::sys::from_diagonal_noise_model_sigmas(sigmas.data.as_mut_slice()),
        }
    }

    pub(crate) fn to_base_model(&self) -> BaseNoiseModel {
        BaseNoiseModel {
            inner: ::sys::cast_diagonal_noise_model_to_base_noise_model(&self.inner),
        }
    }
}
