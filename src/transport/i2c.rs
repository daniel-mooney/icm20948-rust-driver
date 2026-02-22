//! I2C interface for the ICM20948 9-axis IMU

use embedded_hal::i2c::{I2c as I2cTrait, SevenBitAddress, Operation};
use super::{Transport, sealed};

/// I2C transport layer for communicating with an ICM-20948 IMU
///
/// # Type Parameters
/// - `BUS`: An I2C bus implementation (or &mut of) that satisfies [`embedded_hal::i2c::I2c`].
///
/// # Fields
/// - `bus`: The I2C bus
/// - `address`: The devices 7-bit I2C address on `bus`
///
/// # Notes
/// This type does not do any address discovery. You must provide the correct
/// address for your wiring/configuration (e.g. based on the SD0 pin).
pub struct I2cTransport<BUS>
where 
    BUS: I2cTrait
{
    bus: BUS,
    address: SevenBitAddress,
}

impl<BUS> I2cTransport<BUS>
where 
    BUS: I2cTrait
{
    /// Creates a new `I2cTransport`.
    ///
    /// # Parameters
    /// - `bus`: An initialized I2C peripheral implementing [`embedded_hal::i2c::I2c`].
    ///   Ownership is taken to guarantee exclusive access to the bus for this transport.
    /// - `address`: The 7-bit address of the ICM-20948 on the I2C bus.
    ///
    /// # Returns
    /// A new `I2cTransport` configured to communicate with the specified device.
    ///
    /// # Notes
    /// The caller is responsible for ensuring the bus is already configured
    /// (clock speed, pull-ups, etc.) and that the provided address matches the
    /// hardware configuration.
    pub fn new(bus: BUS, address: SevenBitAddress) -> Self {
        Self { bus, address }
    }
}

/// Prevents downstream crates from implementing the `Transport` trait for `I2cTransport`.
impl<BUS> sealed::Sealed for I2cTransport<BUS>
where 
    BUS: I2cTrait,
{}

impl<BUS> Transport for I2cTransport<BUS> 
where 
    BUS: I2cTrait,
{
    /// Error type return by the underlying I2C bus.
    type Error = BUS::Error;

    /// Executes the write-read transaction via the underlying I2C bus.
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.transaction(self.address, &mut [
            Operation::Write(&[addr]),
            Operation::Read(buf),
        ])
    }

    /// Writes the provided bytes directly to the device.
    fn write(&mut self, addr:u8, buf: &[u8]) -> Result<(), Self::Error> {
        self.bus.transaction(self.address, &mut [
            Operation::Write(&[addr]),
            Operation::Write(buf),
        ])
    }
}
