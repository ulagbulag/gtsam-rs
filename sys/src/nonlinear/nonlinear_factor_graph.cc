#include "nonlinear_factor_graph.h"
#include "../base/rust.hpp"

namespace gtsam {

std::unique_ptr<NonlinearFactorGraph> default_nonlinear_factor_graph() {
  return std::make_unique<NonlinearFactorGraph>();
}

void nonlinear_factor_graph_add_prior(
    NonlinearFactorGraph &graph, Key key, const Pose3 &prior,
    const std::shared_ptr<noiseModel::Base> &model) {
  return graph.addPrior(key, prior, to_boost_ptr(model));
}

} // namespace gtsam
