//
//! Defines [`I2cError`].
//

#[doc = crate::_tags!(hw io protocol error)]
/// An I²C bus transaction error.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", enum I2cError),
    test_size_of(I2cError = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum I2cError {
    /// A transmitted address or data byte was not acknowledged.
    Nack,

    /// The transaction exceeded the controller's allowed wait.
    Timeout,

    /// The controller lost bus arbitration.
    ArbitrationLost,
}

crate::impl_trait![fmt::Display+Error for I2cError |self, f| match self {
    Self::Nack => f.write_str("Not acknowledged"),
    Self::Timeout => f.write_str("Transaction timeout"),
    Self::ArbitrationLost => f.write_str("Arbitration lost"),
}];
