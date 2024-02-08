#include "rot3.h"

namespace gtsam {

std::unique_ptr<Rot3> new_rot3() { return std::make_unique<Rot3>(); }

} // namespace gtsam
