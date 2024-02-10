# GTSAM

An unofficial `GTSAM` Rust bindings.

## Supported Features

* LevenbergMarquardtOptimizer
    - for 3D Pose Estimation (`examples/simple_pose_estimation_3d.rs`)

## Dependencies

- boost
- Eigen (optional: required on `build-use-system-eigen` feature enabled)
- metis (optional: required on `build-use-system-metis` feature enabled)

### Install on ArchLinux

```sh
# Note: the following packages are in AUR: metis
paru -S boost eigen metis
```

### Install on Ubuntu

```sh
sudo apt install -y libboost-all-dev libeigen3-dev libmetis-dev
```

## LICENSE

Currently it is under control of `AGPL-3.0`.
However, there are plans to replace the license when the library is ready to establish an ecosystem of use and community-based management.
