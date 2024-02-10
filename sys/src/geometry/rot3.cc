#include "rot3.h"

namespace gtsam {

std::unique_ptr<Rot3> default_rot3() { return std::make_unique<Rot3>(); }

std::unique_ptr<Rot3> from_rot3_quaternion(double w, double x, double y,
                                           double z) {
  return std::make_unique<Rot3>(w, x, y, z);
}

void rot3_to_raw(const Rot3 &src, rust::Slice<double> dst) {
  const auto matrix = src.matrix();

  const double *p_src = matrix.data();
  double *p_dst = dst.data();

  const size_t size = dst.size();
  for (size_t i = 0; i < size; ++i) {
    p_dst[i] = p_src[i];
  }
}

} // namespace gtsam
