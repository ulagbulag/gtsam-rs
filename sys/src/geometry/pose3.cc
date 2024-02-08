#include "pose3.h"

namespace gtsam {

std::unique_ptr<Pose3> new_pose3() { return std::make_unique<Pose3>(); }

} // namespace gtsam
