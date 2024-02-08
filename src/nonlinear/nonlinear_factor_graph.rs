use cxx::UniquePtr;

type Inner = ::sys::nonlinear::nonlinear_factor_graph::NonlinearFactorGraph;

pub struct NonlinearFactorGraph {
    inner: UniquePtr<Inner>,
}

impl Default for NonlinearFactorGraph {
    fn default() -> Self {
        Self {
            inner: ::sys::nonlinear::nonlinear_factor_graph::new_nonlinear_factor_graph(),
        }
    }
}
