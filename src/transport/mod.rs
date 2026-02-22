pub mod i2c;
pub mod spi;

mod sealed {
    pub trait Sealed {}
}

pub trait Transport: sealed::Sealed {
    type Error;

    /// Reads bytes starting at `addr` into `buf`.
    ///
    /// # Contract
    /// - `addr` is the device register address.
    /// - Implementations must attempt to fill the entire buffer.
    /// - The operation is atomic from the caller’s perspective (no partial reads
    ///   unless an error is returned).
    ///
    /// # Errors
    /// Returns any bus-level failure encountered during the transaction.
    fn read(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes bytes to the device.
    ///
    /// # Contract
    /// - `addr` is the device register address.
    /// - `buf` is the byte(s) to write.
    /// - Remaining bytes are written sequentially starting at that register.
    ///
    /// The exact bus transaction is transport-dependent.
    ///
    /// # Errors
    /// Returns any bus-level failure encountered during the transaction.
    fn write(&mut self, addr: u8, buf: &[u8]) -> Result<(), Self::Error>;

}
