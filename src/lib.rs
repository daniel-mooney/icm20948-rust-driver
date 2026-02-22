//! ICM-20948 IMU driver.
//!
//! This crate provides a minimal, transport-agnostic driver for the TDK InvenSense
//! ICM-20948. The public `Icm20948<T>` type is generic over a sealed `Transport`
//! abstraction, allowing the same high-level API to work over I2C (and later SPI)
//! without exposing transport implementation details.
//!
//! ## Architecture
//! - [`register`] defines typed register addresses (and banks) for the device.
//! - [`transport`] defines the sealed [`Transport`] trait.
//! - [`Icm20948`] owns the transport and caches the currently selected user bank
//!   to avoid redundant bank-switch writes.
//!
//! The driver performs all register accesses through [`Icm20948::read_reg`] and
//! [`Icm20948::write_reg`], which ensure the correct register bank is selected
//! before issuing a bus transaction.
//!
//! ## Notes
//! - Accel and gyro outputs are read with a single multi-byte register read,
//!   relying on the fact that the output registers are contiguous.
#![no_std]
use core::result::Result;
use core::option::Option;

use embedded_hal::i2c::{I2c as I2cTrait, SevenBitAddress};
use embedded_hal::spi::SpiDevice as SpiTrait;

pub mod register;
use register::{self as reg, Reg, Bank};
 
mod transport;
use crate::transport::Transport;
use crate::transport::i2c::I2cTransport;
use crate::transport::spi::SpiTransport;

pub mod prelude {
    pub use super::{
        accel,
        gyro,
        mag,
        temp,
        ScaleFactor,
        Reset,
    };
}

pub const ICM20948_I2C_ADDR_L: SevenBitAddress = 0x68;
pub const ICM20948_I2C_ADDR_H: SevenBitAddress = 0x69;
pub const AK09916_I2C_ADDR: Option<SevenBitAddress> = Some(0x0C);
pub const NO_MAG: Option<SevenBitAddress> = None;

/// Convenience types
pub type Icm20948I2c<BUS> = Icm20948<I2cTransport<BUS>>;
pub type Icm20948Spi<BUS> = Icm20948<SpiTransport<BUS>>;

/// A signed 3-axis sensor reading.
///
/// Used for accelerometer, gyroscope (and eventually magnetometer) data.
/// Values are raw device units, as read from the sensor registers and are
/// not scaled by a scale factor to preserve performance on MCU's without an FPU.
/// Scaling is left to the user.
pub struct Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub trait ScaleFactor {
    fn sensitivity(self) -> f32; 
}

/// Accelerometer related constants
pub mod accel {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum FullScale {
        G2,
        G4,
        G8,
        G16,
    }

    impl ScaleFactor for FullScale {
        fn sensitivity(self) -> f32 {
            match self {
                Self::G2 => 16384.0,
                Self::G4 => 8192.0,
                Self::G8 => 4069.0,
                Self::G16 => 2048.0,
            }
        }
    }
}

/// Gyroscope related constants
pub mod gyro {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub enum FullScale {
        DPS250,
        DPS500,
        DPS1000,
        DPS2000,
    }

    impl ScaleFactor for FullScale {
        fn sensitivity(self) -> f32 {
            match self {
                Self::DPS250 => 131.0,
                Self::DPS500 => 65.5,
                Self::DPS1000 => 32.8,
                Self::DPS2000 => 16.4,
            }
        }
    }
}

/// Magnetometer related constants
pub mod mag {
    use super::*;

    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy)]
    pub enum FullScale {
        uT4900,
    }

    impl ScaleFactor for FullScale {
        fn sensitivity(self) -> f32 {
            match self {
                Self::uT4900 => 0.15,
            }
        }
    }
}

pub mod temp {
    use super::*;

    pub enum FullScale {
        DEFAULT,
    }

    impl ScaleFactor for FullScale {
        fn sensitivity(self) -> f32 {
            match self {
                Self::DEFAULT => 333.87
            }
        }
    }
}

/// ICM-20948 driver instance.
///
/// ## Type Parameters
/// `T`: a sealed transport implementation (e.g. an I2C or SPI backend).
///
/// ## Fields
/// `transport`: the transport bus used to communicate with the ICM20948
/// `mag_addr`: the I2C address of the ICM20948's magnetometer (AK09916)
/// `current_bank`: currently selected register address bank
pub struct Icm20948<T>
where 
    T: Transport,
{
    transport: T,
    mag_addr: Option<SevenBitAddress>,
    current_bank: u8,
}

impl<BUS> Icm20948<I2cTransport<BUS>>
where 
    BUS: I2cTrait,
{
    /// Creates a new driver using an I2C bus and 7-bit device address.
    pub fn new(bus: BUS, addr: SevenBitAddress, mag_addr: Option<SevenBitAddress>) -> Icm20948<I2cTransport<BUS>> {
        Self {
            transport: I2cTransport::new(bus, addr),
            mag_addr: mag_addr,
            current_bank: 0,
        }
    }
}

impl<BUS> Icm20948<SpiTransport<BUS>>
where 
    BUS: SpiTrait
{

    /// Creates a new driver using an SPI bus.
    pub fn new(bus: BUS, mag_addr: Option<SevenBitAddress>) -> Icm20948<SpiTransport<BUS>> {
        Self {
            transport: SpiTransport::new(bus),
            mag_addr: mag_addr,
            current_bank: 0,
        }
    }
}

mod sealed {
    use super::Transport;

    pub trait DeviceReset<T>
    where 
        T: Transport,
    {
        /// Device level reset.
        ///
        /// # Notes
        /// - All transport specific reset should be done by implementing the
        /// BusReset trait.
        /// - It is assumed that the transport bus is functional on return.
        fn device_reset(&mut self) -> Result<(), T::Error>;
    }

    pub trait BusReset<T>
    where 
        T: Transport
    {
        /// Transport bus specific reset;
        ///
        /// # Notes
        /// - All device level resets should be done by implementing the
        /// DeviceReset trait.
        /// - It is assumed that the transport bus is functional on return.
        fn bus_reset(&mut self) -> Result<(), T::Error> {
            // Empty default implementation
            Ok(())
        }
    }
}

pub trait Reset<T>: sealed::BusReset<T> + sealed::DeviceReset<T>
where 
    T: Transport
{
    /// Resets the driver and device.
    fn reset(&mut self) -> Result<(), T::Error>;
}

/// Device reset blanket implementation. Device level reset is the same for
/// all ICM20948 + bus type pairs.
impl<T: Transport> sealed::DeviceReset<T> for Icm20948<T>{
    fn device_reset(&mut self) -> Result<(), T::Error> {
        let shift = reg::bank0::pwr_mgmt_1::DEVICE_RESET_SHIFT;
        let buf = [1u8 << shift];
        self.write_reg(reg::bank0::PWR_MGMT_1, &buf)?;

        self.current_bank = 0;

        // Wait until reset bit autoclears
        let mut buf = [0u8];
        loop {
            // Register reading can fail when the Icm20948 is mid-reset.
            // This is usually a "No Acknowledge" type of error. The embedded_hal
            // abstractions do not specify a particular sets of errors, so the best
            // that can be done is to catch all errors and continue. This is not ideal,
            // however, if the earlier `write_reg` call in this method succeeded, then
            // it can best be assumed that the transport will return to a valid state
            // after the ICM20948's internal reset is complete.
            if self.read_reg(reg::bank0::PWR_MGMT_1, &mut buf).is_err() {
                continue;
            }

            let rst_bit = buf[0] & reg::bank0::pwr_mgmt_1::DEVICE_RESET_MASK;

            if rst_bit == 0 {
                break;
            }
        }

        Ok(())
    }
} 

impl<BUS> sealed::BusReset<I2cTransport<BUS>> for Icm20948<I2cTransport<BUS>>
where 
    BUS: I2cTrait,
{ /* "do nothing" Default implementation */ }

impl<BUS> sealed::BusReset<SpiTransport<BUS>> for Icm20948<SpiTransport<BUS>>
where 
    BUS: SpiTrait,
{
    fn bus_reset(&mut self) -> Result<(), <SpiTransport<BUS> as Transport>::Error> {
        // Disable I2C
        self.set_bits(
            reg::bank0::USER_CTRL,
            reg::bank0::user_ctrl::I2C_IF_DIS_MASK,
        )
    }
}

/// Blanket implementation for Reset.
impl<T, DEV> Reset<T> for DEV
where 
    T: Transport,
    DEV: sealed::DeviceReset<T> + sealed::BusReset<T>,
{
    fn reset(&mut self) -> Result<(), <T as Transport>::Error> {
        self.device_reset()?;
        self.bus_reset()
    }
}

impl<T> Icm20948<T> 
where 
    T: Transport,
{
    /// Reads the raw accelerometer output registers.
    ///
    /// Reads 6 bytes starting at `ACCEL_XOUT_H` and converts them from big-endian
    /// register order into signed 16-bit X/Y/Z values.
    pub fn read_accel(&mut self) -> Result<Vec3, T::Error> {
        // Accel regs are contiguous in icm20948. Read mutiple regs at once.
        let mut buf = [0u8; 6];
        self.read_reg(reg::bank0::ACCEL_XOUT_H, &mut buf)?;

        let [xh, xl, yh, yl, zh, zl] = buf;

        let reading = Vec3 {
            x: i16::from_be_bytes([xh, xl]),
            y: i16::from_be_bytes([yh, yl]),
            z: i16::from_be_bytes([zh, zl]),
        };

        Ok(reading)
    }

    pub fn enable_accel(&mut self) -> Result<(), T::Error> {
        self.clear_bits(
            reg::bank0::PWR_MGMT_2,
            reg::bank0::pwr_mgmt_2::DISABLE_ACCEL_MASK,
        )
    }

    pub fn disable_accel(&mut self) -> Result<(), T::Error> {
        self.set_bits(
            reg::bank0::PWR_MGMT_2,
            reg::bank0::pwr_mgmt_2::DISABLE_ACCEL_MASK,
        )
    }

    pub fn set_accel_scale_factor(
        &mut self,
        sf: accel::FullScale
    ) -> Result<(), T::Error> {
        let sf_bits = match sf {
            accel::FullScale::G2 => 0b00,
            accel::FullScale::G4 => 0b01,
            accel::FullScale::G8 => 0b10,
            accel::FullScale::G16 => 0b11,
        };

        let shift = reg::bank2::accel_config::ACCEL_FS_SEL_SHIFT;
        self.write_bits(
            reg::bank2::ACCEL_CONFIG,
            reg::bank2::accel_config::ACCEL_FS_SEL_MASK,
            sf_bits << shift,
        )
    }

    pub fn get_accel_scale_factor(&mut self) -> Result<accel::FullScale, T::Error> {
        let mut buf = [0u8];
        self.read_reg(reg::bank2::ACCEL_CONFIG, &mut buf)?;

        let mask = reg::bank2::accel_config::ACCEL_FS_SEL_MASK;
        let shift = reg::bank2::accel_config::ACCEL_FS_SEL_SHIFT;
        let sf_bits = (buf[0] & mask) >> shift;

        match sf_bits {
            0b00 => Ok(accel::FullScale::G2),
            0b01 => Ok(accel::FullScale::G4),
            0b10 => Ok(accel::FullScale::G8),
            0b11 => Ok(accel::FullScale::G16),
            _ => unreachable!(),
        }
    }

    /// Reads the raw gyroscope output registers.
    ///
    /// Reads 6 bytes starting at `GYRO_XOUT_H` and converts them from big-endian
    /// register order into signed 16-bit X/Y/Z values.
    pub fn read_gyro(&mut self) -> Result<Vec3, T::Error> {
        // Gyro regs are contiguous in icm20948. Read multiple regs at once .
        let mut buf = [0u8; 6];
        self.read_reg(reg::bank0::GYRO_XOUT_H, &mut buf)?;

        let [xh, xl, yh, yl, zh, zl] = buf;

        let reading = Vec3 {
            x: i16::from_be_bytes([xh, xl]),
            y: i16::from_be_bytes([yh, yl]),
            z: i16::from_be_bytes([zh, zl]),
        };

        Ok(reading)
    }

    pub fn enable_gyro(&mut self) -> Result<(), T::Error> {
        self.clear_bits(
            reg::bank0::PWR_MGMT_2,
            reg::bank0::pwr_mgmt_2::DISABLE_GYRO_MASK,
        )
    }

    pub fn disable_gyro(&mut self) -> Result<(), T::Error> {
        self.set_bits(
            reg::bank0::PWR_MGMT_2,
            reg::bank0::pwr_mgmt_2::DISABLE_GYRO_MASK,
        )
    }

    pub fn set_gyro_scale_factor(&mut self, sf: gyro::FullScale) -> Result<(), T::Error> {
        let sf_bits = match sf {
            gyro::FullScale::DPS250 => 0b00,
            gyro::FullScale::DPS500 => 0b01,
            gyro::FullScale::DPS1000 => 0b10,
            gyro::FullScale::DPS2000 => 0b11,
        };

        let shift = reg::bank2::gyro_config_1::GYRO_FS_SEL_SHIFT;
        self.write_bits(
            reg::bank2::GYRO_CONFIG_1,
            reg::bank2::gyro_config_1::GYRO_FS_SEL_MASK,
            sf_bits << shift,
        )
    }

    pub fn get_gyro_scale_factor(&mut self) -> Result<gyro::FullScale, T::Error> {
        let mut buf = [0u8];
        self.read_reg(reg::bank2::GYRO_CONFIG_1, &mut buf)?;

        let mask = reg::bank2::gyro_config_1::GYRO_FS_SEL_MASK;
        let shift = reg::bank2::gyro_config_1::GYRO_FS_SEL_SHIFT;

        let sf_bits = (buf[0] & mask) >> shift;

        match sf_bits {
            0b00 => Ok(gyro::FullScale::DPS250),
            0b01 => Ok(gyro::FullScale::DPS500),
            0b10 => Ok(gyro::FullScale::DPS1000),
            0b11 => Ok(gyro::FullScale::DPS2000),
            _ => unreachable!(),
        }
    }

    pub fn read_6dof(&mut self) -> Result<(Vec3, Vec3), T::Error> {
        let mut buf = [0u8; 12];
        self.read_reg(reg::bank0::ACCEL_XOUT_H, &mut buf)?;

        let accel = Vec3 {
            x: i16::from_be_bytes([buf[0], buf[1]]),
            y: i16::from_be_bytes([buf[2], buf[3]]),
            z: i16::from_be_bytes([buf[4], buf[5]]),
        };

        let gyro = Vec3 {
            x: i16::from_be_bytes([buf[6], buf[7]]),
            y: i16::from_be_bytes([buf[8], buf[9]]),
            z: i16::from_be_bytes([buf[10], buf[11]]),
        };

        Ok((accel, gyro))
    }

    /// Reads the raw magnetometer output registers.
    pub fn read_mag(&mut self) -> Result<Vec3, T::Error> {
        let mut buf = [0u8; 9];
        self.read_reg(reg::bank0::EXT_SLV_SENS_DATA_00, &mut buf)?;

        // buf[0] is status 1 reg
        let reading = Vec3 {
            x: i16::from_le_bytes([buf[1], buf[2]]),
            y: i16::from_le_bytes([buf[3], buf[4]]),
            z: i16::from_le_bytes([buf[5], buf[6]]),
        };

        Ok(reading)
    }

    pub fn enable_mag(&mut self) -> Result<(), T::Error> {
        let mag7 = self.mag_addr.unwrap() & 0x7F;

        // Enable I2C master
        self.set_bits(reg::bank0::USER_CTRL, reg::bank0::user_ctrl::I2C_MST_RST_MASK)?;
        self.set_bits(reg::bank0::USER_CTRL, reg::bank0::user_ctrl::I2C_MST_EN_MASK)?;

        // I2C master clock
        self.write_reg(reg::bank3::I2C_MST_CTRL, &[0x07])?;

        // Soft reset magnetometer
        self.write_reg(reg::bank3::I2C_SLV0_ADDR, &[mag7])?;                 // write
        self.write_reg(reg::bank3::I2C_SLV0_REG, &[reg::mag::CNTL3])?;
        self.write_reg(reg::bank3::I2C_SLV0_DO,  &[0x01])?;                  // SRST=1
        self.write_reg(reg::bank3::I2C_SLV0_CTRL,&[0x80 | 0x01])?;           // enable, 1 byte

        // Set continuous measurement mode 4 (100 Hz)
        self.write_reg(reg::bank3::I2C_SLV0_ADDR, &[mag7])?;                 // write
        self.write_reg(reg::bank3::I2C_SLV0_REG, &[reg::mag::CNTL2])?;
        self.write_reg(reg::bank3::I2C_SLV0_DO,  &[0x08])?;
        self.write_reg(reg::bank3::I2C_SLV0_CTRL,&[0x80 | 0x01])?;           // enable, 1 byte

        // Auto-read: start at ST1 and read 9 bytes: ST1 + HXL..HZH + TMPS + ST2
        let slv0_addr = 0x80 | mag7;
        let slv0_reg  = reg::mag::ST1;
        let slv0_ctrl = 0x80 | 0x09;

        self.write_reg(reg::bank3::I2C_SLV0_ADDR, &[slv0_addr, slv0_reg, slv0_ctrl])?;

        Ok(())
    }

    pub fn get_mag_scale_factor(&mut self) -> Result<mag::FullScale, T::Error> {
        Ok(mag::FullScale::uT4900)
    }

    pub fn read_temp(&mut self) -> Result<i16, T::Error> {
        let mut buf = [0u8, 2];
        self.read_reg(reg::bank0::TEMP_OUT_H, &mut buf)?;

        let reading = i16::from_be_bytes([buf[0], buf[1]]);

        Ok(reading)
    }

    pub fn enable_temp(&mut self) -> Result<(), T::Error> {
        self.clear_bits(
            reg::bank0::PWR_MGMT_1,
            reg::bank0::pwr_mgmt_1::TEMP_DIS_MASK,
        )
    }

    pub fn disable_temp(&mut self) -> Result<(), T::Error> {
        self.set_bits(
            reg::bank0::PWR_MGMT_1,
            reg::bank0::pwr_mgmt_1::TEMP_DIS_MASK,
        )
    }

    pub fn get_temp_scale_factor(&mut self) -> Result<temp::FullScale, T::Error> {
        Ok(temp::FullScale::DEFAULT)
    }

    /// Puts the Icm20948 into sleep mode
    pub fn sleep(&mut self) -> Result<(), T::Error> {
        self.set_bits(
            reg::bank0::PWR_MGMT_1,
            reg::bank0::pwr_mgmt_1::SLEEP_MASK,
        )
    }

    /// Puts the Icm20948 out of sleep mode.
    pub fn awake(&mut self) -> Result<(), T::Error> {
        self.clear_bits(
            reg::bank0::PWR_MGMT_1,
            reg::bank0::pwr_mgmt_1::SLEEP_MASK,
        )
    }

    pub fn set_low_power_mode(&mut self, on: bool) -> Result<(), T::Error> {
        if on {
            self.set_bits(
                reg::bank0::PWR_MGMT_1,
                reg::bank0::pwr_mgmt_1::LP_EN_MASK,
            )
        } else {
            self.clear_bits(
                reg::bank0::PWR_MGMT_1,
                reg::bank0::pwr_mgmt_1::LP_EN_MASK,
            )
        }
    }

    /// Reads bytes from a typed register address into `buf`.
    ///
    /// This automatically selects the correct user bank for `reg` before issuing
    /// a transport read transaction.
    ///
    /// # Contract
    /// Delegates to [`Transport::read`]. In particular, implementations are
    /// expected to fill the entire buffer or return an error.
    pub fn read_reg<B: Bank>(&mut self, reg: Reg<B>, buf: &mut [u8]) -> Result<(), T::Error> {
        self.set_bank(B::ID)?;

        self.transport.read(reg.addr(), buf)?;
        Ok(())
    }

    /// Writes bytes to a typed register address.
    ///
    /// This automatically selects the correct user bank for `reg` before issuing
    /// a transport write transaction.
    ///
    /// # Panics
    /// Panics if `buf.len()` exceeds the internal temporary buffer limit
    /// (`MAX_WRITE_BYTES`).
    ///
    pub fn write_reg<B: Bank>(&mut self, reg: Reg<B>, buf: &[u8]) -> Result<(), T::Error> {
        self.set_bank(B::ID)?;

        self.transport.write(reg.addr(), buf)?;
        Ok(())
    }

    /// Sets the bits set by `mask` in the provided register
    pub fn set_bits<B: Bank>(&mut self, reg: Reg<B>, mask: u8) -> Result<(), T::Error> {
        self.write_bits(reg, mask, mask)
    }

    /// Clears the bits set by `mask` in the provided register
    pub fn clear_bits<B: Bank>(&mut self, reg: Reg<B>, mask: u8) -> Result<(), T::Error> {
        self.write_bits(reg, mask, 0)
    }

    pub fn write_bits<B: Bank>(&mut self, reg: Reg<B>, mask: u8, bits: u8) -> Result<(), T::Error> {
        let mut buf = [0u8];
        self.read_reg(reg, &mut buf)?;

        let field = (buf[0] & !mask) | (bits & mask);
        
        let buf = [field];
        self.write_reg(reg, &buf)?;

        Ok(())
    }

    /// Selects the active user bank if it is not already selected.
    ///
    /// Writes `REG_BANK_SEL` to switch banks and updates the cached
    /// `current_bank`. If the requested bank matches the cached bank, this is a
    /// no-op.
    fn set_bank(&mut self, bank: u8) -> Result<(), T::Error> {
        // IMPORTANT:
        // This method cannot use `write_reg` for writing as `write_reg`
        // depends on `set_bank`.
        if self.current_bank == bank {
            return Ok(());
        }

        // REG_BANK_SEL address is the same for all banks
        let addr = reg::bank0::REG_BANK_SEL.addr();
        let bank_shift = reg::bank0::reg_bank_sel::USER_BANK_SHIFT;

        let data = [bank << bank_shift];
        self.transport.write(addr, &data)?;

        self.current_bank = bank;
        Ok(())
    }
}
