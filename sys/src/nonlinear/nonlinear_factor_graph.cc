#include "nonlinear_factor_graph.h"
#include "../base/rust.hpp"

namespace gtsam {

std::unique_ptr<NonlinearFactorGraph> default_nonlinear_factor_graph() {
  return std::make_unique<NonlinearFactorGraph>();
}

void nonlinear_factor_graph_add_between_factor_pose3(
    NonlinearFactorGraph &graph, Key key1, Key key2, const Pose3 &measured,
    const std::shared_ptr<noiseModel::Base> &model) {
  return graph.add(
      BetweenFactorPose3(key1, key2, measured, to_boost_ptr(model)));
}

void nonlinear_factor_graph_add_prior_factor_pose3(
    NonlinearFactorGraph &graph, Key key, const Pose3 &prior,
    const std::shared_ptr<noiseModel::Base> &model) {
  return graph.addPrior(key, prior, to_boost_ptr(model));
}

} // namespace gtsam
