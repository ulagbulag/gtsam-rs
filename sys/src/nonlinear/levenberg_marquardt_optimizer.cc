#include "levenberg_marquardt_optimizer.h"
#include <algorithm>
#include <memory>

namespace gtsam {

std::unique_ptr<LevenbergMarquardtOptimizer>
new_levenberg_marquardt_optimizer(const NonlinearFactorGraph &graph,
                                  const Values &initial_values,
                                  const LevenbergMarquardtParams &params) {
  return std::make_unique<LevenbergMarquardtOptimizer>(graph, initial_values,
                                                       params);
}

} // namespace gtsam
