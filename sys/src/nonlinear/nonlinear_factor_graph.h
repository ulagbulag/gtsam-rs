#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Pose3.h>
#include <gtsam/nonlinear/NonlinearFactor.h>
#include <gtsam/nonlinear/NonlinearFactorGraph.h>
#include <memory>

namespace gtsam {

std::unique_ptr<NonlinearFactorGraph> default_nonlinear_factor_graph();

void nonlinear_factor_graph_add_prior(
    NonlinearFactorGraph &graph, Key key, const Pose3 &prior,
    const std::shared_ptr<noiseModel::Base> &model);

} // namespace gtsam
