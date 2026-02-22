#![no_std]


#![no_main]

use panic_serial as _;
use arduino_hal::{self as hal, prelude::*};

use icm20948::{Icm20948I2c, ICM20948_I2C_ADDR_L, AK09916_I2C_ADDR, prelude::*};

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);


    panic_serial::impl_panic_handler!(
        // This is the type of the UART port to use for printing the message:
        arduino_hal::usart::Usart<
        arduino_hal::pac::USART0,
        arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PD0>,
        arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PD1>
        >
    );

    let serial = hal::default_serial!(dp, pins, 57600);
    let serial = share_serial_port_with_panic(serial);

    let mut i2c = hal::I2c::with_external_pullup(
        dp.TWI,
        pins.a4.into_floating_input(),
        pins.a5.into_floating_input(),
        50000,
    );
    
    ufmt::uwriteln!(serial, "I2C initialised\r").unwrap_infallible();

    match i2c.ping_device(ICM20948_I2C_ADDR_L, hal::i2c::Direction::Write) {
        Ok(true) => {},
        Ok(false) => panic!("Cannot detect Icm20948\r"),
        _ => panic!("Bus Error\r"),
    };

    let mut dev = Icm20948I2c::new(
        i2c,
        ICM20948_I2C_ADDR_L,
        AK09916_I2C_ADDR,
    );

    dev.reset().unwrap();

    // Must call awake first otherwise magnetometer cfg will not work.
    dev.awake().unwrap();

    dev.enable_accel().unwrap();
    dev.set_accel_scale_factor(accel::FullScale::G2).unwrap();

    dev.enable_gyro().unwrap();
    dev.set_gyro_scale_factor(gyro::FullScale::DPS250).unwrap();

    dev.enable_mag().unwrap();
    dev.enable_temp().unwrap();
    
    ufmt::uwriteln!(serial, "ICM20948 configured\r").unwrap_infallible();

    // heart beat led
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        let (accel, gyro) = dev.read_6dof().unwrap();
        let mag = dev.read_mag().unwrap();
        let temp_unnorm = dev.read_temp().unwrap();

        ufmt::uwriteln!(serial, "accel: {{x: {}, y: {}, z: {}}}\r", accel.x, accel.y, accel.z).unwrap_infallible();
        ufmt::uwriteln!(serial, "gyro:  {{x: {}, y: {}, z: {}}}\r", gyro.x, gyro.y, gyro.z).unwrap_infallible();
        ufmt::uwriteln!(serial, "mag:   {{x: {}, y: {}, z: {}}}\r", mag.x, mag.y, mag.z).unwrap_infallible();
        ufmt::uwriteln!(serial, "temp:  {}\r", temp_unnorm).unwrap_infallible();
        
        led.toggle();
        hal::delay_ms(200);
    }
}
