#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Pose3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Pose3> new_pose3();

} // namespace gtsam
