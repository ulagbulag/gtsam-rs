#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Pose3.h>
#include <gtsam/nonlinear/NonlinearFactor.h>
#include <gtsam/nonlinear/NonlinearFactorGraph.h>
#include <gtsam/slam/BetweenFactor.h>
#include <memory>

namespace gtsam {

typedef BetweenFactor<Pose3> BetweenFactorPose3;

std::unique_ptr<NonlinearFactorGraph> default_nonlinear_factor_graph();

void nonlinear_factor_graph_add_between_factor_pose3(
    NonlinearFactorGraph &graph, Key from, Key to, const Pose3 &prior,
    const std::shared_ptr<noiseModel::Base> &model);

void nonlinear_factor_graph_add_prior_factor_pose3(
    NonlinearFactorGraph &graph, Key key, const Pose3 &prior,
    const std::shared_ptr<noiseModel::Base> &model);

} // namespace gtsam
