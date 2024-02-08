#pragma once

#include "rust/cxx.h"
#include <gtsam/nonlinear/NonlinearFactorGraph.h>
#include <memory>

namespace gtsam {

std::unique_ptr<NonlinearFactorGraph> new_nonlinear_factor_graph();

} // namespace gtsam
