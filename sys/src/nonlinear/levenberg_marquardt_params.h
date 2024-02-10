#pragma once

#include "rust/cxx.h"
#include <gtsam/nonlinear/LevenbergMarquardtParams.h>
#include <memory>

namespace gtsam {

std::unique_ptr<LevenbergMarquardtParams> default_levenberg_marquardt_params();

void levenberg_marquardt_params_set_max_iterations(
    LevenbergMarquardtParams &params, uint32_t value);

} // namespace gtsam
