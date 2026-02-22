//! An example file whichi reads data from the ICM20948 using SPI

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*, rcc};
use stm32f4xx_hal::spi::{self, Spi};

use embedded_hal_bus::spi::ExclusiveDevice;

use icm20948::{Icm20948Spi, AK09916_I2C_ADDR, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let clk_cfg = rcc::Config::hsi().sysclk(96.MHz());
    let mut rcc = dp.RCC.freeze(clk_cfg);

    let mut delay = cp.SYST.delay(&mut rcc.clocks);

    // Setup SPI
    let spi_mode = spi::Mode {
        polarity: spi::Polarity::IdleHigh,
        phase: spi::Phase::CaptureOnSecondTransition,
    };

    let gpioa = dp.GPIOA.split(&mut rcc);

    let sck = gpioa.pa5;
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7;
    let cs = gpioa.pa0.into_push_pull_output();

    let spi_bus = Spi::new(
        dp.SPI1,
        (Some(sck), Some(miso), Some(mosi)),
        spi_mode,
        1.MHz(),
        &mut rcc,
    );

    let spi_dev = ExclusiveDevice::new_no_delay(spi_bus, cs).unwrap();
    
    println!("SPI Initialised");

    // ICM20948 setup
    let mut dev = Icm20948Spi::new(spi_dev, AK09916_I2C_ADDR);

    // Optionally reset device to default state.
    dev.reset().unwrap();
    dev.awake().unwrap();

    // Initial device configuration
    dev.enable_accel().unwrap();
    dev.set_accel_scale_factor(accel::FullScale::G2).unwrap();

    dev.enable_gyro().unwrap();
    dev.set_gyro_scale_factor(gyro::FullScale::DPS250).unwrap();

    println!("Icm20948 Initialised");

    // Accelerometer scale factor sensitivity i.e. G per i16
    let a_sens = dev.get_accel_scale_factor().unwrap().sensitivity();
    let g_sens = dev.get_gyro_scale_factor().unwrap().sensitivity();

    loop {
        delay.delay_ms(200);

        let (accel, gyro) = dev.read_6dof().unwrap();

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
