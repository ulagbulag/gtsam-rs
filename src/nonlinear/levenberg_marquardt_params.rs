use cxx::UniquePtr;

pub struct LevenbergMarquardtParams {
    pub(super) inner: UniquePtr<::sys::LevenbergMarquardtParams>,
}

impl Default for LevenbergMarquardtParams {
    fn default() -> Self {
        Self {
            inner: ::sys::default_levenberg_marquardt_params(),
        }
    }
}

impl LevenbergMarquardtParams {
    pub fn set_max_iterations(&mut self, value: u32) {
        ::sys::levenberg_marquardt_params_set_max_iterations(self.inner.pin_mut(), value)
    }
}
