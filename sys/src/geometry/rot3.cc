#include "rot3.h"

namespace gtsam {

std::unique_ptr<Rot3> default_rot3() { return std::make_unique<Rot3>(); }

std::unique_ptr<Rot3> from_rot3_quaternion(double w, double x, double y,
                                           double z) {
  return std::make_unique<Rot3>(w, x, y, z);
}

} // namespace gtsam
