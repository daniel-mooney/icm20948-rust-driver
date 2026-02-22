//! ICM-20948 bank-typed register addresses.
//!
//! The ICM-20948 register map is split into USER BANK 0..3, and numeric register addresses can
//! overlap between banks. `Reg<BankN>` encodes the bank at the type level so registers from
//! different banks cannot be mixed accidentally.

#![allow(dead_code)]

use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Bank0;

#[derive(Clone, Copy)]
pub struct Bank1;

#[derive(Clone, Copy)]
pub struct Bank2;

#[derive(Clone, Copy)]
pub struct Bank3;

/// Marker trait for ICM-20948 USER BANKs (0..=3).
pub trait Bank: Clone + Copy {
    const ID: u8;
}

impl Bank for Bank0 { const ID: u8 = 0; }
impl Bank for Bank1 { const ID: u8 = 1; }
impl Bank for Bank2 { const ID: u8 = 2; }
impl Bank for Bank3 { const ID: u8 = 3; }

/// Register address tagged with its USER BANK at the type level.
///
/// # Type Parameters
/// - `B`: The bank the register belongs to
///
/// # Fields
/// - `addr`: The address of the register on the device.
#[derive(Clone, Copy)]
pub struct Reg<B: Bank> {
    addr: u8,
    _bank: PhantomData<B>,
}

impl<B: Bank> Reg<B> {
    /// Create a bank-types register address.
    pub const fn new(addr: u8) -> Self {
        Self { addr, _bank: PhantomData }
    }

    /// Get the register's address.
    pub const fn addr(self) -> u8 {
        self.addr
    }
    
    /// Get the register's bank id.
    pub const fn bank(self) -> u8 {
        B::ID
    }
}

pub mod bank0 {
    use super::*;

    pub const WHO_AM_I: Reg<Bank0> = Reg::new(0x00);

    /// USER_CTRL register address + bitmasks
    pub const USER_CTRL: Reg<Bank0> = Reg::new(0x03);
    pub mod user_ctrl {
        pub const I2C_MST_RST_MASK: u8 = 1u8 << 1;
        pub const I2C_MST_RST_SHIFT: u8 = 1;

        pub const SRAM_RST_MASK: u8 = 1u8 << 2;
        pub const SRAM_RST_SHIFT: u8 = 2;

        pub const DMP_RST_MASK: u8 = 1u8 << 3;
        pub const DMP_RST_SHIFT: u8 = 3;

        pub const I2C_IF_DIS_MASK: u8 = 1u8 << 4;
        pub const I2C_IF_DIS_SHIFT: u8 = 4;

        pub const I2C_MST_EN_MASK: u8 = 1u8 << 5;
        pub const I2C_MST_EN_SHIFT: u8 = 5;

        pub const FIFO_EN_MASK: u8 = 1u8 << 6;
        pub const FIFO_EN_SHIFT: u8 = 6;

        pub const DMP_EN_MASK: u8 = 1u8 << 7;
        pub const DMP_EN_SHIFT: u8 = 7;
    }

    /// LP_CONFIG register address + bitmasks
    pub const LP_CONFIG: Reg<Bank0> = Reg::new(0x05);
    pub mod lp_config {
        pub const GYRO_CYCLE_MASK: u8 = 1u8 << 4;
        pub const GYRO_CYCLE_SHIFT: u8 = 4;

        pub const ACCEL_CYCLE_MASK: u8 = 1u8 << 5;
        pub const ACCEL_CYCLE_SHIFT: u8 = 5;

        pub const I2C_MST_CYCLE_MASK: u8 = 1u8 << 6;
        pub const I2C_MST_CYCLE_SHIFT: u8 = 6;
    }

    /// PWR_MGMT_1 register address + bitmasks
    pub const PWR_MGMT_1: Reg<Bank0> = Reg::new(0x06);
    pub mod pwr_mgmt_1 {
        pub const CLKSEL_MASK: u8 = 0x07;
        pub const CLKSEL_SHIFT: u8 = 0;

        pub const TEMP_DIS_MASK: u8 = 1u8 << 3;
        pub const TEMP_DIS_SHIFT: u8 = 3;

        pub const LP_EN_MASK: u8 = 1u8 << 5;
        pub const LP_EN_SHIFT: u8 = 5;

        pub const SLEEP_MASK: u8 = 1u8 << 6;
        pub const SLEEP_SHIFT: u8 = 6;

        pub const DEVICE_RESET_MASK: u8 = 1u8 << 7;
        pub const DEVICE_RESET_SHIFT: u8 = 7;
    }

    /// PWR_MGMT_2 register address + bitmasks
    pub const PWR_MGMT_2: Reg<Bank0> = Reg::new(0x07);
    pub mod pwr_mgmt_2 {
        pub const DISABLE_GYRO_MASK: u8 = 0x07;
        pub const DISABLE_GYRO_SHIFT: u8 = 0;

        pub const DISABLE_ACCEL_MASK: u8 = 0x38;
        pub const DISABLE_ACCEL_SHIFT: u8 = 3;
    }

    /// INT_PIN_CFG register address + bitmasks
    pub const INT_PIN_CFG: Reg<Bank0> = Reg::new(0x0F);
    pub mod int_pin_cfg {
        pub const BYPASS_EN_MASK: u8 = 1u8 << 1;
        pub const BYPASS_EN_SHIFT: u8 = 1;

        pub const FSYNC_INT_MODE_EN_MASK: u8 = 1u8 << 2;
        pub const FSYNC_INT_MODE_EN_SHIFT: u8 = 2;

        pub const ACTL_FSYNC_MASK: u8 = 1u8 << 3;
        pub const ACTL_FSYNC_SHIFT: u8 = 3;

        pub const INT_ANYRD_2CLEAR_MASK: u8 = 1u8 << 4;
        pub const INT_ANYRD_2CLEAR_SHIFT: u8 = 4;

        pub const INT1_LATCH_INT_EN_MASK: u8 = 1u8 << 5;
        pub const INT1_LATCH_INT_EN_SHIFT: u8 = 5;

        pub const INT1_OPEN_MASK: u8 = 1u8 << 6;
        pub const INT1_OPEN_SHIFT: u8 = 6;

        pub const INT1_ACTL_MASK: u8 = 1u8 << 7;
        pub const INT1_ACTL_SHIFT: u8 = 7;
    }

    /// INT_ENABLE register address + bitmasks
    pub const INT_ENABLE: Reg<Bank0> = Reg::new(0x10);
    pub mod int_enable {
        pub const I2C_MST_INT_EN_MASK: u8 = 1u8 << 0;
        pub const I2C_MST_INT_EN_SHIFT: u8 = 0;

        pub const DMP_INT1_EN_MASK: u8 = 1u8 << 1;
        pub const DMP_INT1_EN_SHIFT: u8 = 1;

        pub const PLL_RDY_EN_MASK: u8 = 1u8 << 4;
        pub const PLL_RDY_EN_SHIFT: u8 = 4;

        pub const WOM_INT_EN_MASK: u8 = 1u8 << 5;
        pub const WOM_INT_EN_SHIFT: u8 = 5;

        pub const REG_WOF_EN_MASK: u8 = 1u8 << 7;
        pub const REG_WOF_EN_SHIFT: u8 = 7;
    }

    /// INT_ENABLE_1 register address + bitmasks
    pub const INT_ENABLE_1: Reg<Bank0> = Reg::new(0x11);
    pub mod int_enable_1 {
        pub const RAW_DATA_0_RDY_EN_MASK: u8 = 1u8 << 0;
        pub const RAW_DATA_0_RDY_EN_SHIFT: u8 = 0;
    }

    /// INT_ENABLE_2 register address + bitmasks
    pub const INT_ENABLE_2: Reg<Bank0> = Reg::new(0x12);
    pub mod int_enable_2 {
        pub const FIFO_OVERFLOW_EN_MASK: u8 = 0x1F;
        pub const FIFO_OVERFLOW_EN_SHIFT: u8 = 0;
    }

    /// INT_ENABLE_3 register address + bitmasks
    pub const INT_ENABLE_3: Reg<Bank0> = Reg::new(0x13);
    pub mod int_enable_3 {
        pub const FIFO_WM_EN_MASK: u8 = 0x1F;
        pub const FIFO_WM_EN_SHIFT: u8 = 0;
    }

    /// I2C_MST_STATUS register address + bitmasks
    pub const I2C_MST_STATUS: Reg<Bank0> = Reg::new(0x17);
    pub mod i2c_mst_status {
        pub const I2C_SLV0_NACK_MASK: u8 = 1u8 << 0;
        pub const I2C_SLV0_NACK_SHIFT: u8 = 0;

        pub const I2C_SLV1_NACK_MASK: u8 = 1u8 << 1;
        pub const I2C_SLV1_NACK_SHIFT: u8 = 1;

        pub const I2C_SLV2_NACK_MASK: u8 = 1u8 << 2;
        pub const I2C_SLV2_NACK_SHIFT: u8 = 2;

        pub const I2C_SLV3_NACK_MASK: u8 = 1u8 << 3;
        pub const I2C_SLV3_NACK_SHIFT: u8 = 3;

        pub const I2C_SLV4_NACK_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV4_NACK_SHIFT: u8 = 4;

        pub const I2C_LOST_ARB_MASK: u8 = 1u8 << 5;
        pub const I2C_LOST_ARB_SHIFT: u8 = 5;

        pub const I2C_SLV4_DONE_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV4_DONE_SHIFT: u8 = 6;

        pub const PASS_THROUGH_MASK: u8 = 1u8 << 7;
        pub const PASS_THROUGH_SHIFT: u8 = 7;
    }

    /// INT_STATUS register address + bitmasks
    pub const INT_STATUS: Reg<Bank0> = Reg::new(0x19);
    pub mod int_status {
        pub const I2C_MST_INT_MASK: u8 = 1u8 << 0;
        pub const I2C_MST_INT_SHIFT: u8 = 0;

        pub const DMP_INT1_MASK: u8 = 1u8 << 1;
        pub const DMP_INT1_SHIFT: u8 = 1;

        pub const PLL_RDY_INT_MASK: u8 = 1u8 << 2;
        pub const PLL_RDY_INT_SHIFT: u8 = 2;

        pub const WOM_INT_MASK: u8 = 1u8 << 3;
        pub const WOM_INT_SHIFT: u8 = 3;
    }

    /// INT_STATUS_1 register address + bitmasks
    pub const INT_STATUS_1: Reg<Bank0> = Reg::new(0x1A);
    pub mod int_status_1 {
        pub const RAW_DATA_0_RDY_INT_MASK: u8 = 1u8 << 0;
        pub const RAW_DATA_0_RDY_INT_SHIFT: u8 = 0;
    }

    /// INT_STATUS_2 register address + bitmasks
    pub const INT_STATUS_2: Reg<Bank0> = Reg::new(0x1B);
    pub mod int_status_2 {
        pub const FIFO_OVERFLOW_INT_MASK: u8 = 0x1F;
        pub const FIFO_OVERFLOW_INT_SHIFT: u8 = 0;
    }

    /// INT_STATUS_3 register address + bitmasks
    pub const INT_STATUS_3: Reg<Bank0> = Reg::new(0x1C);
    pub mod int_status_3 {
        pub const FIFO_WM_INT_MASK: u8 = 0x1F;
        pub const FIFO_WM_INT_SHIFT: u8 = 0;
    }

    pub const DELAY_TIMEH: Reg<Bank0> = Reg::new(0x28);
    pub const DELAY_TIMEL: Reg<Bank0> = Reg::new(0x29);

    pub const ACCEL_XOUT_H: Reg<Bank0> = Reg::new(0x2D);
    pub const ACCEL_XOUT_L: Reg<Bank0> = Reg::new(0x2E);
    pub const ACCEL_YOUT_H: Reg<Bank0> = Reg::new(0x2F);
    pub const ACCEL_YOUT_L: Reg<Bank0> = Reg::new(0x30);
    pub const ACCEL_ZOUT_H: Reg<Bank0> = Reg::new(0x31);
    pub const ACCEL_ZOUT_L: Reg<Bank0> = Reg::new(0x32);

    pub const GYRO_XOUT_H: Reg<Bank0> = Reg::new(0x33);
    pub const GYRO_XOUT_L: Reg<Bank0> = Reg::new(0x34);
    pub const GYRO_YOUT_H: Reg<Bank0> = Reg::new(0x35);
    pub const GYRO_YOUT_L: Reg<Bank0> = Reg::new(0x36);
    pub const GYRO_ZOUT_H: Reg<Bank0> = Reg::new(0x37);
    pub const GYRO_ZOUT_L: Reg<Bank0> = Reg::new(0x38);

    pub const TEMP_OUT_H: Reg<Bank0> = Reg::new(0x39);
    pub const TEMP_OUT_L: Reg<Bank0> = Reg::new(0x3A);

    pub const EXT_SLV_SENS_DATA_00: Reg<Bank0> = Reg::new(0x3B);
    pub const EXT_SLV_SENS_DATA_01: Reg<Bank0> = Reg::new(0x3C);
    pub const EXT_SLV_SENS_DATA_02: Reg<Bank0> = Reg::new(0x3D);
    pub const EXT_SLV_SENS_DATA_03: Reg<Bank0> = Reg::new(0x3E);
    pub const EXT_SLV_SENS_DATA_04: Reg<Bank0> = Reg::new(0x3F);
    pub const EXT_SLV_SENS_DATA_05: Reg<Bank0> = Reg::new(0x40);
    pub const EXT_SLV_SENS_DATA_06: Reg<Bank0> = Reg::new(0x41);
    pub const EXT_SLV_SENS_DATA_07: Reg<Bank0> = Reg::new(0x42);
    pub const EXT_SLV_SENS_DATA_08: Reg<Bank0> = Reg::new(0x43);
    pub const EXT_SLV_SENS_DATA_09: Reg<Bank0> = Reg::new(0x44);
    pub const EXT_SLV_SENS_DATA_10: Reg<Bank0> = Reg::new(0x45);
    pub const EXT_SLV_SENS_DATA_11: Reg<Bank0> = Reg::new(0x46);
    pub const EXT_SLV_SENS_DATA_12: Reg<Bank0> = Reg::new(0x47);
    pub const EXT_SLV_SENS_DATA_13: Reg<Bank0> = Reg::new(0x48);
    pub const EXT_SLV_SENS_DATA_14: Reg<Bank0> = Reg::new(0x49);
    pub const EXT_SLV_SENS_DATA_15: Reg<Bank0> = Reg::new(0x4A);
    pub const EXT_SLV_SENS_DATA_16: Reg<Bank0> = Reg::new(0x4B);
    pub const EXT_SLV_SENS_DATA_17: Reg<Bank0> = Reg::new(0x4C);
    pub const EXT_SLV_SENS_DATA_18: Reg<Bank0> = Reg::new(0x4D);
    pub const EXT_SLV_SENS_DATA_19: Reg<Bank0> = Reg::new(0x4E);
    pub const EXT_SLV_SENS_DATA_20: Reg<Bank0> = Reg::new(0x4F);
    pub const EXT_SLV_SENS_DATA_21: Reg<Bank0> = Reg::new(0x50);
    pub const EXT_SLV_SENS_DATA_22: Reg<Bank0> = Reg::new(0x51);
    pub const EXT_SLV_SENS_DATA_23: Reg<Bank0> = Reg::new(0x52);

    /// FIFO_EN_1 register address + bitmasks
    pub const FIFO_EN_1: Reg<Bank0> = Reg::new(0x66);
    pub mod fifo_en_1 {
        pub const SLV_0_FIFO_EN_MASK: u8 = 1u8 << 0;
        pub const SLV_0_FIFO_EN_SHIFT: u8 = 0;

        pub const SLV_1_FIFO_EN_MASK: u8 = 1u8 << 1;
        pub const SLV_1_FIFO_EN_SHIFT: u8 = 1;

        pub const SLV_2_FIFO_EN_MASK: u8 = 1u8 << 2;
        pub const SLV_2_FIFO_EN_SHIFT: u8 = 2;

        pub const SLV_3_FIFO_EN_MASK: u8 = 1u8 << 3;
        pub const SLV_3_FIFO_EN_SHIFT: u8 = 3;
    }

    /// FIFO_EN_2 register address + bitmasks
    pub const FIFO_EN_2: Reg<Bank0> = Reg::new(0x67);
    pub mod fifo_en_2 {
        pub const TEMP_FIFO_EN_MASK: u8 = 1u8 << 0;
        pub const TEMP_FIFO_EN_SHIFT: u8 = 0;

        pub const GYRO_X_FIFO_EN_MASK: u8 = 1u8 << 1;
        pub const GYRO_X_FIFO_EN_SHIFT: u8 = 1;

        pub const GYRO_Y_FIFO_EN_MASK: u8 = 1u8 << 2;
        pub const GYRO_Y_FIFO_EN_SHIFT: u8 = 2;

        pub const GYRO_Z_FIFO_EN_MASK: u8 = 1u8 << 3;
        pub const GYRO_Z_FIFO_EN_SHIFT: u8 = 3;

        pub const ACCEL_FIFO_EN_MASK: u8 = 1u8 << 4;
        pub const ACCEL_FIFO_EN_SHIFT: u8 = 4;
    }

    /// FIFO_RST register address + bitmasks
    pub const FIFO_RST: Reg<Bank0> = Reg::new(0x68);
    pub mod fifo_rst {
        pub const FIFO_RESET_MASK: u8 = 0x1F;
        pub const FIFO_RESET_SHIFT: u8 = 0;
    }

    /// FIFO_MODE register address + bitmasks
    pub const FIFO_MODE: Reg<Bank0> = Reg::new(0x69);
    pub mod fifo_mode {
        pub const FIFO_MODE_MASK: u8 = 0x1F;
        pub const FIFO_MODE_SHIFT: u8 = 0;
    }

    /// FIFO_COUNTH register address + bitmasks
    pub const FIFO_COUNTH: Reg<Bank0> = Reg::new(0x70);
    pub mod fifo_counth {
        pub const FIFO_CNT_12_8_MASK: u8 = 0x1F;
        pub const FIFO_CNT_12_8_SHIFT: u8 = 0;
    }

    pub const FIFO_COUNTL: Reg<Bank0> = Reg::new(0x71);
    pub const FIFO_R_W: Reg<Bank0> = Reg::new(0x72);

    /// DATA_RDY_STATUS register address + bitmasks
    pub const DATA_RDY_STATUS: Reg<Bank0> = Reg::new(0x74);
    pub mod data_rdy_status {
        pub const RAW_DATA_RDY_MASK: u8 = 0x0F;
        pub const RAW_DATA_RDY_SHIFT: u8 = 0;

        pub const WOF_STATUS_MASK: u8 = 1u8 << 7;
        pub const WOF_STATUS_SHIFT: u8 = 7;
    }

    pub const FIFO_CFG: Reg<Bank0> = Reg::new(0x76);

    /// REG_BANK_SEL register address + bitmasks
    pub const REG_BANK_SEL: Reg<Bank0> = Reg::new(0x7F);
    pub mod reg_bank_sel {
        pub const USER_BANK_MASK: u8 = 0x30;
        pub const USER_BANK_SHIFT: u8 = 4;
    }
}

pub mod bank1 {
    use super::*;

    pub const SELF_TEST_X_GYRO: Reg<Bank1> = Reg::new(0x02);
    pub const SELF_TEST_Y_GYRO: Reg<Bank1> = Reg::new(0x03);
    pub const SELF_TEST_Z_GYRO: Reg<Bank1> = Reg::new(0x04);

    pub const SELF_TEST_X_ACCEL: Reg<Bank1> = Reg::new(0x0E);
    pub const SELF_TEST_Y_ACCEL: Reg<Bank1> = Reg::new(0x0F);
    pub const SELF_TEST_Z_ACCEL: Reg<Bank1> = Reg::new(0x10);

    pub const XA_OFFS_H: Reg<Bank1> = Reg::new(0x14);

    /// XA_OFFS_L register address + bitmasks
    pub const XA_OFFS_L: Reg<Bank1> = Reg::new(0x15);
    pub mod xa_offs_l {
        pub const XA_OFFS_6_0_MASK: u8 = 0x7F;
        pub const XA_OFFS_6_0_SHIFT: u8 = 0;
    }

    pub const YA_OFFS_H: Reg<Bank1> = Reg::new(0x17);

    /// YA_OFFS_L register address + bitmasks
    pub const YA_OFFS_L: Reg<Bank1> = Reg::new(0x18);
    pub mod ya_offs_l {
        pub const YA_OFFS_6_0_MASK: u8 = 0x7F;
        pub const YA_OFFS_6_0_SHIFT: u8 = 0;
    }

    pub const ZA_OFFS_H: Reg<Bank1> = Reg::new(0x1A);

    /// ZA_OFFS_L register address + bitmasks
    pub const ZA_OFFS_L: Reg<Bank1> = Reg::new(0x1B);
    pub mod za_offs_l {
        pub const ZA_OFFS_6_0_MASK: u8 = 0x7F;
        pub const ZA_OFFS_6_0_SHIFT: u8 = 0;
    }

    pub const TIMEBASE_CORRECTION_PLL: Reg<Bank1> = Reg::new(0x28);

    /// REG_BANK_SEL register address + bitmasks
    pub const REG_BANK_SEL: Reg<Bank1> = Reg::new(0x7F);
    pub mod reg_bank_sel {
        pub const USER_BANK_MASK: u8 = 0x30;
        pub const USER_BANK_SHIFT: u8 = 4;
    }
}

pub mod bank2 {
    use super::*;

    pub const GYRO_SMPLRT_DIV: Reg<Bank2> = Reg::new(0x00);

    /// GYRO_CONFIG_1 register address + bitmasks
    pub const GYRO_CONFIG_1: Reg<Bank2> = Reg::new(0x01);
    pub mod gyro_config_1 {
        pub const GYRO_FCHOICE_MASK: u8 = 1u8 << 0;
        pub const GYRO_FCHOICE_SHIFT: u8 = 0;

        pub const GYRO_FS_SEL_MASK: u8 = 0x06;
        pub const GYRO_FS_SEL_SHIFT: u8 = 1;

        pub const GYRO_DLPFCFG_MASK: u8 = 0x38;
        pub const GYRO_DLPFCFG_SHIFT: u8 = 3;
    }

    /// GYRO_CONFIG_2 register address + bitmasks
    pub const GYRO_CONFIG_2: Reg<Bank2> = Reg::new(0x02);
    pub mod gyro_config_2 {
        pub const GYRO_AVGCFG_MASK: u8 = 0x07;
        pub const GYRO_AVGCFG_SHIFT: u8 = 0;

        pub const ZGYRO_CTEN_MASK: u8 = 1u8 << 3;
        pub const ZGYRO_CTEN_SHIFT: u8 = 3;

        pub const YGYRO_CTEN_MASK: u8 = 1u8 << 4;
        pub const YGYRO_CTEN_SHIFT: u8 = 4;

        pub const XGYRO_CTEN_MASK: u8 = 1u8 << 5;
        pub const XGYRO_CTEN_SHIFT: u8 = 5;
    }

    pub const XG_OFFS_USRH: Reg<Bank2> = Reg::new(0x03);
    pub const XG_OFFS_USRL: Reg<Bank2> = Reg::new(0x04);
    pub const YG_OFFS_USRH: Reg<Bank2> = Reg::new(0x05);
    pub const YG_OFFS_USRL: Reg<Bank2> = Reg::new(0x06);
    pub const ZG_OFFS_USRH: Reg<Bank2> = Reg::new(0x07);
    pub const ZG_OFFS_USRL: Reg<Bank2> = Reg::new(0x08);

    /// ODR_ALIGN_EN register address + bitmasks
    pub const ODR_ALIGN_EN: Reg<Bank2> = Reg::new(0x09);
    pub mod odr_align_en {
        pub const ODR_ALIGN_EN_MASK: u8 = 1u8 << 0;
        pub const ODR_ALIGN_EN_SHIFT: u8 = 0;
    }

    /// ACCEL_SMPLRT_DIV_1 register address + bitmasks
    pub const ACCEL_SMPLRT_DIV_1: Reg<Bank2> = Reg::new(0x10);
    pub mod accel_smplrt_div_1 {
        pub const ACCEL_SMPLRT_DIV_11_8_MASK: u8 = 0x0F;
        pub const ACCEL_SMPLRT_DIV_11_8_SHIFT: u8 = 0;
    }

    pub const ACCEL_SMPLRT_DIV_2: Reg<Bank2> = Reg::new(0x11);

    /// ACCEL_INTEL_CTRL register address + bitmasks
    pub const ACCEL_INTEL_CTRL: Reg<Bank2> = Reg::new(0x12);
    pub mod accel_intel_ctrl {
        pub const ACCEL_INTEL_MODE_INT_MASK: u8 = 1u8 << 0;
        pub const ACCEL_INTEL_MODE_INT_SHIFT: u8 = 0;

        pub const ACCEL_INTEL_EN_MASK: u8 = 1u8 << 1;
        pub const ACCEL_INTEL_EN_SHIFT: u8 = 1;
    }

    pub const ACCEL_WOM_THR: Reg<Bank2> = Reg::new(0x13);

    /// ACCEL_CONFIG register address + bitmasks
    pub const ACCEL_CONFIG: Reg<Bank2> = Reg::new(0x14);
    pub mod accel_config {
        pub const ACCEL_FCHOICE_MASK: u8 = 1u8 << 0;
        pub const ACCEL_FCHOICE_SHIFT: u8 = 0;

        pub const ACCEL_FS_SEL_MASK: u8 = 0x06;
        pub const ACCEL_FS_SEL_SHIFT: u8 = 1;

        pub const ACCEL_DLPFCFG_MASK: u8 = 0x38;
        pub const ACCEL_DLPFCFG_SHIFT: u8 = 3;
    }

    /// ACCEL_CONFIG_2 register address + bitmasks
    pub const ACCEL_CONFIG_2: Reg<Bank2> = Reg::new(0x15);
    pub mod accel_config_2 {
        pub const DEC3_CFG_MASK: u8 = 0x03;
        pub const DEC3_CFG_SHIFT: u8 = 0;

        pub const AZ_ST_EN_REG_MASK: u8 = 1u8 << 3;
        pub const AZ_ST_EN_REG_SHIFT: u8 = 3;

        pub const AY_ST_EN_REG_MASK: u8 = 1u8 << 4;
        pub const AY_ST_EN_REG_SHIFT: u8 = 4;

        pub const AX_ST_EN_REG_MASK: u8 = 1u8 << 5;
        pub const AX_ST_EN_REG_SHIFT: u8 = 5;
    }

    /// FSYNC_CONFIG register address + bitmasks
    pub const FSYNC_CONFIG: Reg<Bank2> = Reg::new(0x52);
    pub mod fsync_config {
        pub const EXT_SYNC_SET_MASK: u8 = 0x0F;
        pub const EXT_SYNC_SET_SHIFT: u8 = 0;

        pub const WOF_EDGE_INT_MASK: u8 = 1u8 << 4;
        pub const WOF_EDGE_INT_SHIFT: u8 = 4;

        pub const WOF_DEGLITCH_EN_MASK: u8 = 1u8 << 5;
        pub const WOF_DEGLITCH_EN_SHIFT: u8 = 5;

        pub const DELAY_TIME_EN_MASK: u8 = 1u8 << 7;
        pub const DELAY_TIME_EN_SHIFT: u8 = 7;
    }

    /// TEMP_CONFIG register address + bitmasks
    pub const TEMP_CONFIG: Reg<Bank2> = Reg::new(0x53);
    pub mod temp_config {
        pub const TEMP_DLPFCFG_MASK: u8 = 0x07;
        pub const TEMP_DLPFCFG_SHIFT: u8 = 0;
    }

    /// MOD_CTRL_USR register address + bitmasks
    pub const MOD_CTRL_USR: Reg<Bank2> = Reg::new(0x54);
    pub mod mod_ctrl_usr {
        pub const REG_LP_DMP_EN_MASK: u8 = 1u8 << 0;
        pub const REG_LP_DMP_EN_SHIFT: u8 = 0;
    }

    /// REG_BANK_SEL register address + bitmasks
    pub const REG_BANK_SEL: Reg<Bank2> = Reg::new(0x7F);
    pub mod reg_bank_sel {
        pub const USER_BANK_MASK: u8 = 0x30;
        pub const USER_BANK_SHIFT: u8 = 4;
    }
}

pub mod bank3 {
    use super::*;

    /// I2C_MST_ODR_CONFIG register address + bitmasks
    pub const I2C_MST_ODR_CONFIG: Reg<Bank3> = Reg::new(0x00);
    pub mod i2c_mst_odr_config {
        pub const I2C_MST_ODR_CONFIG_MASK: u8 = 0x0F;
        pub const I2C_MST_ODR_CONFIG_SHIFT: u8 = 0;
    }

    /// I2C_MST_CTRL register address + bitmasks
    pub const I2C_MST_CTRL: Reg<Bank3> = Reg::new(0x01);
    pub mod i2c_mst_ctrl {
        pub const I2C_MST_CLK_MASK: u8 = 0x0F;
        pub const I2C_MST_CLK_SHIFT: u8 = 0;

        pub const I2C_MST_P_NSR_MASK: u8 = 1u8 << 4;
        pub const I2C_MST_P_NSR_SHIFT: u8 = 4;

        pub const MULT_MST_EN_MASK: u8 = 1u8 << 7;
        pub const MULT_MST_EN_SHIFT: u8 = 7;
    }

    /// I2C_MST_DELAY_CTRL register address + bitmasks
    pub const I2C_MST_DELAY_CTRL: Reg<Bank3> = Reg::new(0x02);
    pub mod i2c_mst_delay_ctrl {
        pub const I2C_SLV0_DELAY_EN_MASK: u8 = 1u8 << 0;
        pub const I2C_SLV0_DELAY_EN_SHIFT: u8 = 0;

        pub const I2C_SLV1_DELAY_EN_MASK: u8 = 1u8 << 1;
        pub const I2C_SLV1_DELAY_EN_SHIFT: u8 = 1;

        pub const I2C_SLV2_DELAY_EN_MASK: u8 = 1u8 << 2;
        pub const I2C_SLV2_DELAY_EN_SHIFT: u8 = 2;

        pub const I2C_SLV3_DELAY_EN_MASK: u8 = 1u8 << 3;
        pub const I2C_SLV3_DELAY_EN_SHIFT: u8 = 3;

        pub const I2C_SLV4_DELAY_EN_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV4_DELAY_EN_SHIFT: u8 = 4;

        pub const DELAY_ES_SHADOW_MASK: u8 = 1u8 << 7;
        pub const DELAY_ES_SHADOW_SHIFT: u8 = 7;
    }

    /// I2C_SLV0_ADDR register address + bitmasks
    pub const I2C_SLV0_ADDR: Reg<Bank3> = Reg::new(0x03);
    pub mod i2c_slv0_addr {
        pub const I2C_ID_0_MASK: u8 = 0x7F;
        pub const I2C_ID_0_SHIFT: u8 = 0;

        pub const I2C_SLV0_RNW_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV0_RNW_SHIFT: u8 = 7;
    }

    pub const I2C_SLV0_REG: Reg<Bank3> = Reg::new(0x04);

    /// I2C_SLV0_CTRL register address + bitmasks
    pub const I2C_SLV0_CTRL: Reg<Bank3> = Reg::new(0x05);
    pub mod i2c_slv0_ctrl {
        pub const I2C_SLV0_LENG_MASK: u8 = 0x0F;
        pub const I2C_SLV0_LENG_SHIFT: u8 = 0;

        pub const I2C_SLV0_GRP_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV0_GRP_SHIFT: u8 = 4;

        pub const I2C_SLV0_REG_DIS_MASK: u8 = 1u8 << 5;
        pub const I2C_SLV0_REG_DIS_SHIFT: u8 = 5;

        pub const I2C_SLV0_BYTE_SW_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV0_BYTE_SW_SHIFT: u8 = 6;

        pub const I2C_SLV0_EN_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV0_EN_SHIFT: u8 = 7;
    }

    pub const I2C_SLV0_DO: Reg<Bank3> = Reg::new(0x06);

    /// I2C_SLV1_ADDR register address + bitmasks
    pub const I2C_SLV1_ADDR: Reg<Bank3> = Reg::new(0x07);
    pub mod i2c_slv1_addr {
        pub const I2C_ID_1_MASK: u8 = 0x7F;
        pub const I2C_ID_1_SHIFT: u8 = 0;

        pub const I2C_SLV1_RNW_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV1_RNW_SHIFT: u8 = 7;
    }

    pub const I2C_SLV1_REG: Reg<Bank3> = Reg::new(0x08);

    /// I2C_SLV1_CTRL register address + bitmasks
    pub const I2C_SLV1_CTRL: Reg<Bank3> = Reg::new(0x09);
    pub mod i2c_slv1_ctrl {
        pub const I2C_SLV1_LENG_MASK: u8 = 0x0F;
        pub const I2C_SLV1_LENG_SHIFT: u8 = 0;

        pub const I2C_SLV1_GRP_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV1_GRP_SHIFT: u8 = 4;

        pub const I2C_SLV1_REG_DIS_MASK: u8 = 1u8 << 5;
        pub const I2C_SLV1_REG_DIS_SHIFT: u8 = 5;

        pub const I2C_SLV1_BYTE_SW_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV1_BYTE_SW_SHIFT: u8 = 6;

        pub const I2C_SLV1_EN_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV1_EN_SHIFT: u8 = 7;
    }

    pub const I2C_SLV1_DO: Reg<Bank3> = Reg::new(0x0A);

    /// I2C_SLV2_ADDR register address + bitmasks
    pub const I2C_SLV2_ADDR: Reg<Bank3> = Reg::new(0x0B);
    pub mod i2c_slv2_addr {
        pub const I2C_ID_2_MASK: u8 = 0x7F;
        pub const I2C_ID_2_SHIFT: u8 = 0;

        pub const I2C_SLV2_RNW_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV2_RNW_SHIFT: u8 = 7;
    }

    pub const I2C_SLV2_REG: Reg<Bank3> = Reg::new(0x0C);

    /// I2C_SLV2_CTRL register address + bitmasks
    pub const I2C_SLV2_CTRL: Reg<Bank3> = Reg::new(0x0D);
    pub mod i2c_slv2_ctrl {
        pub const I2C_SLV2_LENG_MASK: u8 = 0x0F;
        pub const I2C_SLV2_LENG_SHIFT: u8 = 0;

        pub const I2C_SLV2_GRP_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV2_GRP_SHIFT: u8 = 4;

        pub const I2C_SLV2_REG_DIS_MASK: u8 = 1u8 << 5;
        pub const I2C_SLV2_REG_DIS_SHIFT: u8 = 5;

        pub const I2C_SLV2_BYTE_SW_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV2_BYTE_SW_SHIFT: u8 = 6;

        pub const I2C_SLV2_EN_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV2_EN_SHIFT: u8 = 7;
    }

    pub const I2C_SLV2_DO: Reg<Bank3> = Reg::new(0x0E);

    /// I2C_SLV3_ADDR register address + bitmasks
    pub const I2C_SLV3_ADDR: Reg<Bank3> = Reg::new(0x0F);
    pub mod i2c_slv3_addr {
        pub const I2C_ID_3_MASK: u8 = 0x7F;
        pub const I2C_ID_3_SHIFT: u8 = 0;

        pub const I2C_SLV3_RNW_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV3_RNW_SHIFT: u8 = 7;
    }

    pub const I2C_SLV3_REG: Reg<Bank3> = Reg::new(0x10);

    /// I2C_SLV3_CTRL register address + bitmasks
    pub const I2C_SLV3_CTRL: Reg<Bank3> = Reg::new(0x11);
    pub mod i2c_slv3_ctrl {
        pub const I2C_SLV3_LENG_MASK: u8 = 0x0F;
        pub const I2C_SLV3_LENG_SHIFT: u8 = 0;

        pub const I2C_SLV3_GRP_MASK: u8 = 1u8 << 4;
        pub const I2C_SLV3_GRP_SHIFT: u8 = 4;

        pub const I2C_SLV3_REG_DIS_MASK: u8 = 1u8 << 5;
        pub const I2C_SLV3_REG_DIS_SHIFT: u8 = 5;

        pub const I2C_SLV3_BYTE_SW_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV3_BYTE_SW_SHIFT: u8 = 6;

        pub const I2C_SLV3_EN_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV3_EN_SHIFT: u8 = 7;
    }

    pub const I2C_SLV3_DO: Reg<Bank3> = Reg::new(0x12);

    /// I2C_SLV4_ADDR register address + bitmasks
    pub const I2C_SLV4_ADDR: Reg<Bank3> = Reg::new(0x13);
    pub mod i2c_slv4_addr {
        pub const I2C_ID_4_MASK: u8 = 0x7F;
        pub const I2C_ID_4_SHIFT: u8 = 0;

        pub const I2C_SLV4_RNW_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV4_RNW_SHIFT: u8 = 7;
    }

    pub const I2C_SLV4_REG: Reg<Bank3> = Reg::new(0x14);

    /// I2C_SLV4_CTRL register address + bitmasks
    pub const I2C_SLV4_CTRL: Reg<Bank3> = Reg::new(0x15);
    pub mod i2c_slv4_ctrl {
        pub const I2C_SLV4_DLY_MASK: u8 = 0x1F;
        pub const I2C_SLV4_DLY_SHIFT: u8 = 0;

        pub const I2C_SLV4_REG_DIS_MASK: u8 = 1u8 << 5;
        pub const I2C_SLV4_REG_DIS_SHIFT: u8 = 5;

        pub const I2C_SLV4_BYTE_SW_MASK: u8 = 1u8 << 6;
        pub const I2C_SLV4_BYTE_SW_SHIFT: u8 = 6;

        pub const I2C_SLV4_EN_MASK: u8 = 1u8 << 7;
        pub const I2C_SLV4_EN_SHIFT: u8 = 7;
    }

    pub const I2C_SLV4_DO: Reg<Bank3> = Reg::new(0x16);
    pub const I2C_SLV4_DI: Reg<Bank3> = Reg::new(0x17);

    /// REG_BANK_SEL register address + bitmasks
    pub const REG_BANK_SEL: Reg<Bank3> = Reg::new(0x7F);
    pub mod reg_bank_sel {
        pub const USER_BANK_MASK: u8 = 0x30;
        pub const USER_BANK_SHIFT: u8 = 4;
    }
}

/// Register address for the ICM20948's magnetometer.
///
/// These address are not banked.
pub mod mag {
    pub const WIA2: u8 = 0x01;
    pub mod wia2 {
        pub const WIA2_MASK: u8 = 0xFF;
        pub const WIA2_SHIFT: u8 = 0;

        pub const AK09916_ID: u8 = 0x09;
    }

    /// ST1 register address + bitmasks
    pub const ST1: u8 = 0x10;
    pub mod st1 {
        pub const DRDY_MASK: u8 = 0x01;
        pub const DRDY_SHIFT: u8 = 0;

        pub const DOR_MASK: u8 = 0x02;
        pub const DOR_SHIFT: u8 = 1;
    }

    pub const HXL: u8 = 0x11;
    pub const HXH: u8 = 0x12;
    pub const HYL: u8 = 0x13;
    pub const HYH: u8 = 0x14;
    pub const HZL: u8 = 0x15;
    pub const HZH: u8 = 0x16;

    /// ST2 register address + bitmasks
    pub const ST2: u8 = 0x18;
    pub mod st2 {
        pub const HOFL_MASK: u8 = 0x08;
        pub const HOFL_SHIFT: u8 = 3;
    }

    /// CNTL2 register address + bitmasks
    pub const CNTL2: u8 = 0x31;
    pub mod cntl2 {
        pub const MODE_MASK: u8 = 0x1F;
        pub const MODE_SHIFT: u8 = 0;

        pub const MODE_POWER_DOWN: u8 = 0b00000;
        pub const MODE_SINGLE: u8 = 0b00001;
        pub const MODE_CONTINUOUS_1: u8 = 0b00010;
        pub const MODE_CONTINUOUS_2: u8 = 0b00100;
        pub const MODE_CONTINUOUS_3: u8 = 0b00110;
        pub const MODE_CONTINUOUS_4: u8 = 0b01000;
        pub const MODE_SELF_TEST: u8 = 0b10000;
    }

    /// CNTL3 register address + bitmasks
    pub const CNTL3: u8 = 0x32;
    pub mod cntl3 {
        pub const SRST_MASK: u8 = 0x01;
        pub const SRST_SHIFT: u8 = 0;
    }

    pub const TS1: u8 = 0x33;
    pub const TS2: u8 = 0x34;
}
