// devela/src/media/font/format/bdf/_parse/number.rs
//
//! Defines `BdfNumber`.
//

use super::BdfResult;
use crate::{BdfError as E, is, lets, unwrap};

/// Exact normalized BDF `number`.
///
/// Its value is `coefficient × 10⁻ˢᶜᵃˡᵉ`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(super) struct BdfNumber {
    coefficient: i64,
    scale: u8,
}
impl BdfNumber {
    pub(super) const ZERO: Self = Self { coefficient: 0, scale: 0 };

    pub(super) const fn new(mut coefficient: i64, mut scale: u8) -> Self {
        while scale != 0 && coefficient % 10 == 0 {
            coefficient /= 10;
            scale -= 1;
        }
        Self { coefficient, scale }
    }
    pub(super) const fn is_positive(self) -> bool {
        self.coefficient > 0
    }
    pub(super) const fn parse(bytes: &[u8], line: u32) -> BdfResult<Self> {
        is! { bytes.is_empty(), return Err(E::invalid_value(line)) }
        lets! { negative = bytes[0] == b'-', mut i = is! { negative, 1, 0 } }
        is! { i == bytes.len(), return Err(E::invalid_value(line)) }
        lets! { mut magnitude = 0_u64, mut scale = 0_u8, mut digits = 0_usize, mut decimal = false }
        while i < bytes.len() {
            let byte = bytes[i];
            if byte == b'.' {
                is! { decimal, return Err(E::invalid_value(line)) }
                decimal = true;
            } else if byte >= b'0' && byte <= b'9' {
                let digit = (byte - b'0') as u64;
                magnitude = unwrap![some_or magnitude.checked_mul(10),
                    return Err(E::invalid_value(line))];
                magnitude = unwrap![some_or magnitude.checked_add(digit),
                    return Err(E::invalid_value(line))];
                digits += 1;
                if decimal {
                    scale = unwrap![some_or scale.checked_add(1),
                        return Err(E::invalid_value(line))];
                }
            } else {
                return Err(E::invalid_value(line));
            }
            i += 1;
        }
        is! { digits == 0, return Err(E::invalid_value(line)) }
        let coefficient = if negative {
            const MIN_MAGNITUDE: u64 = i64::MAX as u64 + 1;
            if magnitude > MIN_MAGNITUDE {
                return Err(E::invalid_value(line));
            } else if magnitude == MIN_MAGNITUDE {
                i64::MIN
            } else {
                -(magnitude as i64)
            }
        } else {
            is! { magnitude > i64::MAX as u64, return Err(E::invalid_value(line)) }
            magnitude as i64
        };
        Ok(Self::new(coefficient, scale))
    }
}
