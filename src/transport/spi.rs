//! SPI interface for the ICM20948 9-axis IMU

use embedded_hal::spi::{Operation, SpiDevice};
use super::{sealed, Transport};

/// SPI transport layer for communicating with an ICM-20948 IMU
///
/// # Type Parameters
/// - `BUS`: An SPI device implementation (or &mut of) that satisfies [`embedded_hal::spi::SpiDevice`].
///
/// # Fields
/// - `bus`: The SPI device
///
/// # Notes
/// This transport assumes the ICM-20948 SPI convention where bit 7 of the first byte is
/// the read flag. [`Transport::read`] sets this bit (`0x80 | addr`) and then performs a
/// single SPI transaction that writes the address byte followed by reading into `buf`,
/// keeping chip-select asserted for the duration.
pub struct SpiTransport<BUS>
where
    BUS: SpiDevice,
{
    bus: BUS,
}

impl<BUS> SpiTransport<BUS>
where
    BUS: SpiDevice,
{
    /// Creates a new `SpiTransport`.
    ///
    /// # Parameters
    /// - `bus`: An initialized SPI device implementing [`embedded_hal::spi::SpiDevice`].
    ///   Ownership is taken to guarantee exclusive access to the device for this transport.
    ///
    /// # Returns
    /// A new `SpiTransport` configured to communicate with the device.
    ///
    /// # Notes
    /// The caller is responsible for ensuring the SPI peripheral is already configured
    /// (mode, clock speed, etc.) for the ICM-20948.
    pub fn new(bus: BUS) -> Self {
        Self { bus }
    }
}

/// Prevents downstream crates from implementing the `Transport` trait for `SpiTransport`.
impl<BUS> sealed::Sealed for SpiTransport<BUS> where BUS: SpiDevice {}

impl<BUS> Transport for SpiTransport<BUS>
where
    BUS: SpiDevice,
{
    /// Error type return by the underlying SPI device.
    type Error = BUS::Error;

    /// Executes the address + read transaction via the underlying SPI device.
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error> {
        let addr = 0x80 | addr;     // Add read bit
        self.bus.transaction(&mut [
            Operation::Write(&[addr]),
            Operation::Read(buf),
        ])
    }

    /// Writes the provided bytes directly to the device.
    fn write(&mut self, addr: u8, buf: &[u8]) -> Result<(), Self::Error> {
        self.bus.transaction(&mut [
            Operation::Write(&[addr]),
            Operation::Write(buf),
        ])
    }
}
