#pragma once

#include "rust/cxx.h"
#include <gtsam/inference/Symbol.h>
#include <memory>

namespace gtsam {

std::unique_ptr<Symbol> default_symbol();

std::unique_ptr<Symbol> new_symbol(uint64_t index);

} // namespace gtsam