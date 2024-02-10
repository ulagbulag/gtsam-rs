#pragma once

#include "rust/cxx.h"
#include <gtsam/nonlinear/LevenbergMarquardtOptimizer.h>
#include <gtsam/nonlinear/LevenbergMarquardtParams.h>
#include <gtsam/nonlinear/NonlinearFactorGraph.h>
#include <gtsam/nonlinear/Values.h>
#include <memory>

namespace gtsam {

std::unique_ptr<LevenbergMarquardtOptimizer>
new_levenberg_marquardt_optimizer(const NonlinearFactorGraph &graph,
                                  const Values &initial_values,
                                  const LevenbergMarquardtParams &params);

} // namespace gtsam
