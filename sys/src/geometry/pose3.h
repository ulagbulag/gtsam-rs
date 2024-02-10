#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Point3.h>
#include <gtsam/geometry/Pose3.h>
#include <gtsam/geometry/Rot3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Pose3> default_pose3();

std::unique_ptr<Pose3> new_pose3(const Rot3 &rotation, const Point3 &point);

const Rot3 &pose3_rotation(const Pose3 &pose);

const Point3 &pose3_translation(const Pose3 &pose);

} // namespace gtsam
