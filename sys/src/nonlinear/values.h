#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Pose3.h>
#include <gtsam/nonlinear/Values.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Values> default_values();

const Pose3 &values_at_pose3(const Values &values, Key key);

bool values_exists(const Values &values, Key key);

void values_insert_pose3(Values &values, Key key, const Pose3 &value);

} // namespace gtsam
