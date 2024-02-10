#include "values.h"

namespace gtsam {

std::unique_ptr<Values> default_values() { return std::make_unique<Values>(); }

const Pose3 &values_at_pose3(const Values &values, Key key) {
  return values.at(key).cast<Pose3>();
}

bool values_exists(const Values &values, Key key) { return values.exists(key); }

void values_insert_pose3(Values &values, Key key, const Pose3 &value) {
  return values.insert(key, value);
}

} // namespace gtsam
