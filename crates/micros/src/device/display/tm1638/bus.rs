//
//! Defines [`Tm1638Bus`].
//

#[doc = crate::_tags!(hw io protocol)]
/// Transaction access to a TM1638 serial interface.
#[doc = crate::_doc_meta!{
    location("device/display", trait Tm1638Bus),
}]
/// A transaction holds `STB` active across all supplied bytes.
///
/// Implementations own the physical details of the TM1638 three-wire
/// interface: `STB`, `CLK`, bidirectional `DIO`, LSB-first shifting,
/// bus turnaround, and timing.
pub trait Tm1638Bus {
    /// Error returned by the interface.
    type Error;

    /// Writes byte slices as one uninterrupted TM1638 transaction.
    fn write_slices(&mut self, slices: &[&[u8]]) -> Result<(), Self::Error>;

    /// Writes bytes and then reads bytes in one uninterrupted transaction.
    ///
    /// This includes the `DIO` output-to-input turnaround required for
    /// key-scan reads.
    fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes one byte slice as one transaction.
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write_slices(&[bytes])
    }
}
