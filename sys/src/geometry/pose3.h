#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Point3.h>
#include <gtsam/geometry/Pose3.h>
#include <gtsam/geometry/Rot3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Pose3> default_pose3();

std::unique_ptr<Pose3> new_pose3(const Rot3 &rotation, const Point3 &point);

} // namespace gtsam
