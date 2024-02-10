#pragma once

#include "rust/cxx.h"
#include <gtsam/geometry/Rot3.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Rot3> default_rot3();

std::unique_ptr<Rot3> from_rot3_quaternion(double w, double x, double y,
                                           double z);

void rot3_to_raw(const Rot3 &src, rust::Slice<double> dst);

} // namespace gtsam
