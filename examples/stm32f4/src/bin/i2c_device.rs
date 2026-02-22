//! An example file which reads data from the ICM20948.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, pac, prelude::*, rcc, i2c::I2c};

use icm20948::{Icm20948I2c, ICM20948_I2C_ADDR_L, AK09916_I2C_ADDR, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let clk_cfg = rcc::Config::hsi().sysclk(96.MHz());
    let mut rcc = dp.RCC.freeze(clk_cfg);

    let mut delay = cp.SYST.delay(&mut rcc.clocks);

    let gpiob = dp.GPIOB.split(&mut rcc);

    let sda = gpiob.pb7;
    let scl = gpiob.pb8;

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        hal::i2c::Mode::standard(100.kHz()),
        &mut rcc,
    );

    println!("I2C initialised");

    let mut icm20948 = Icm20948I2c::new(
        i2c,
        ICM20948_I2C_ADDR_L,
        AK09916_I2C_ADDR,
    );
    
    // Optionally reset device to default state.
    icm20948.reset().unwrap();

    // Initial device configuration
    icm20948.enable_accel().unwrap();
    icm20948.set_accel_scale_factor(accel::FullScale::G2).unwrap();

    icm20948.enable_gyro().unwrap();
    icm20948.set_gyro_scale_factor(gyro::FullScale::DPS250).unwrap();

    icm20948.awake().unwrap();
    
    println!("ICM20948 Initialised");

    // Accelerometer scale factor sensitivity i.e. G per i16
    let a_sens = icm20948.get_accel_scale_factor().unwrap().sensitivity();
    let g_sens = icm20948.get_gyro_scale_factor().unwrap().sensitivity();

    loop {
        delay.delay_ms(200);

        let accel = icm20948.read_accel().unwrap();
        let gyro = icm20948.read_gyro().unwrap();
        
        // STM32F411 has a FPU
        let ax = (accel.x as f32) / a_sens;
        let ay = (accel.y as f32) / a_sens;
        let az = (accel.z as f32) / a_sens;

        let gx = (gyro.x as f32) / g_sens;
        let gy = (gyro.y as f32) / g_sens;
        let gz = (gyro.z as f32) / g_sens;

       println!(
            "accel: {{x: {=f32}, y: {=f32}, z: {=f32}}} gyro: {{x: {=f32}, y: {=f32}, z: {=f32}}}",
            ax, ay, az, gx, gy, gz
        );
    }
}
