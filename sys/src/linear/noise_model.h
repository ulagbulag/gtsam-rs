#pragma once

#include "rust/cxx.h"
#include <gtsam/linear/NoiseModel.h>
#include <memory>

namespace gtsam {

typedef noiseModel::Base BaseNoiseModel;
typedef noiseModel::Diagonal DiagonalNoiseModel;

std::shared_ptr<DiagonalNoiseModel>
from_diagonal_noise_model_sigmas(const rust::Slice<double> sigmas);

std::shared_ptr<BaseNoiseModel> cast_diagonal_noise_model_to_base_noise_model(
    const std::shared_ptr<DiagonalNoiseModel> &a);

} // namespace gtsam
