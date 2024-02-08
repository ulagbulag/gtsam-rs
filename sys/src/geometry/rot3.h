#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Rot3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Rot3> new_rot3();

} // namespace gtsam
