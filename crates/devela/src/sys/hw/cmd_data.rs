//
//! Defines [`CmdDataWrite`].
//

#[doc = crate::_tags!(hw io protocol)]
/// Write access to separate command and data byte streams.
#[doc = crate::_doc_meta!{
    location("sys/hw", enum CmdDataWrite),
}]
/// Abstracts an interface that distinguishes control commands from payload
/// data independently of the physical transport used to carry them.
pub trait CmdDataWrite {
    /// Error returned by the underlying interface.
    type Error;

    /// Writes one sequence of command bytes.
    fn write_cmd(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    /// Writes one sequence of data bytes.
    fn write_data(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}
