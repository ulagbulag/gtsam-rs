#include "point3.h"

namespace gtsam {

std::unique_ptr<Point3> default_point3() { return std::make_unique<Point3>(); }

std::unique_ptr<Point3> new_point3(double x, double y, double z) {
  return std::make_unique<Point3>(x, y, z);
}

} // namespace gtsam
