//
//! Defines [`Ssd13xxWrite`].
//

#[doc = crate::_tags!(hw io protocol)]
/// Write access to an SSD13xx display interface.
#[doc = crate::_doc_meta!{
    location("device/display", trait Ssd13xxWrite),
}]
/// Separates controller commands from display-RAM data independently of
/// the underlying physical interface.
///
/// I²C, SPI, and other supported interfaces can implement this capability.
pub trait Ssd13xxWrite {
    /// Error returned by the interface.
    type Error;

    /// Writes one sequence of controller commands.
    fn write_commands(&mut self, commands: &[u8]) -> Result<(), Self::Error>;

    /// Writes display-RAM data.
    fn write_data(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}
