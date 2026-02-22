# ICM20948 Low-Power 9-Axis MEMS IMU

The ICM20948 is a low-power 9-axis MEMS IMU with a built-in Digital Motion Processor (DMP). Refer to the [official product page](https://invensense.tdk.com/products/motion-tracking/9-axis/icm-20948/) for complete documentation and datasheets.

This repository provides a platform- and transport-layer agnostic Rust driver for the ICM20948. It supports both SPI and I2C via `embedded-hal` traits. See the included examples for usage guidance.

This project was developed as a mechanism to learn embedded Rust. The driver is fully functional, however, for a more comprehensive implementation, including async support, consider [icm-20948-rs by 1-rafael-1](https://github.com/1-rafael-1/icm-20948-rs), which is better suited for production use.

If a simpler, synchronous implementation is sufficient, this repository may be appropriate. Refer to the examples for integration details.
