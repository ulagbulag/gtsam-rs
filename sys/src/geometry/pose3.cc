#include "pose3.h"

namespace gtsam {

std::unique_ptr<Pose3> default_pose3() { return std::make_unique<Pose3>(); }

std::unique_ptr<Pose3> new_pose3(const Rot3 &rotation, const Point3 &point) {
  return std::make_unique<Pose3>(rotation, point);
}

const Rot3 &pose3_rotation(const Pose3 &pose) { return pose.rotation(); }

const Point3 &pose3_translation(const Pose3 &pose) {
  return pose.translation();
}

} // namespace gtsam
