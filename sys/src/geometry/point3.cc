#include "point3.h"

namespace gtsam {

std::unique_ptr<Point3> default_point3() { return std::make_unique<Point3>(); }

std::unique_ptr<Point3> new_point3(double x, double y, double z) {
  return std::make_unique<Point3>(x, y, z);
}

void point3_to_raw(const Point3 &src, rust::Slice<double> dst) {
  const double *p_src = src.data();
  double *p_dst = dst.data();

  const size_t size = dst.size();
  for (size_t i = 0; i < size; ++i) {
    p_dst[i] = p_src[i];
  }
}

} // namespace gtsam
