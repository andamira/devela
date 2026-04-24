// devela::sys::device::display::x11::surface::cpu
//
//! Defines [`XCpuBuffer`].
//

use crate::{Vec, XDisplay, XError, XImageStore, vec_ as vec};

#[doc = crate::_tags!(unix runtime)]
/// Cpu-backed pixel buffer for X11 image upload.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct XCpuBuffer {
    width: u16,
    height: u16,
    depth: u8,
    bytes: Vec<u8>,
}
impl XCpuBuffer {
    /// Creates a new cpu pixel buffer compatible with the display.
    pub fn new(display: &XDisplay, width: u16, height: u16, depth: u8) -> Result<Self, XError> {
        let len = display.image_format.len_bytes(width, height);
        Ok(Self { width, height, depth, bytes: vec![0; len] })
    }
}
#[rustfmt::skip]
impl XImageStore for XCpuBuffer {
    fn size(&self) -> (u16, u16) { (self.width, self.height) }
    fn depth(&self) -> u8 { self.depth }
    fn bytes(&self) -> &[u8] { &self.bytes }
    fn bytes_mut(&mut self) -> &mut [u8] { &mut self.bytes }
    fn resize(&mut self, display: &XDisplay, width: u16, height: u16, depth: u8)
        -> Result<(), XError> {
        let len = display.image_format.len_bytes(width, height);
        self.bytes.resize(len, 0);
        self.width = width;
        self.height = height;
        self.depth = depth;
        Ok(())
    }
}
