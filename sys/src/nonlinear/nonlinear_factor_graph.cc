#include "nonlinear_factor_graph.h"

namespace gtsam {

std::unique_ptr<NonlinearFactorGraph> new_nonlinear_factor_graph() {
  return std::make_unique<NonlinearFactorGraph>();
}

} // namespace gtsam
