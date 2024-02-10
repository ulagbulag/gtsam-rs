#include "levenberg_marquardt_params.h"

namespace gtsam {

std::unique_ptr<LevenbergMarquardtParams> default_levenberg_marquardt_params() {
  return std::make_unique<LevenbergMarquardtParams>();
}

void levenberg_marquardt_params_set_max_iterations(
    LevenbergMarquardtParams &params, uint32_t value) {
  return params.setMaxIterations(value);
}

} // namespace gtsam
