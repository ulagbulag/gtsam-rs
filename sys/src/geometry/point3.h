#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Point3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Point3> default_point3();

std::unique_ptr<Point3> new_point3(double x, double y, double z);

} // namespace gtsam
