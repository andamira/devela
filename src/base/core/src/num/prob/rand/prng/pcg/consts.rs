// devela_base_core::num::prob::rand::pcg::consts
//
//

#![expect(unused)] // WIP

// default PCG multipliers
pub(crate) const PCG_MUL_8: u32 = 141;
pub(crate) const PCG_MUL_16: u32 = 12829;
pub(crate) const PCG_MUL_32: u32 = 747796405;
pub(crate) const PCG_MUL_64: u64 = 6364136223846793005;
pub(crate) const PCG_MUL_128: u128 = 47026247687942121848144207491837523525;

// default PCG increments
pub(crate) const PCG_INC_8: u32 = 77;
pub(crate) const PCG_INC_16: u32 = 47989;
pub(crate) const PCG_INC_32: u32 = 2891336453;
pub(crate) const PCG_INC_64: u64 = 1442695040888963407;
pub(crate) const PCG_INC_128: u128 = 117397592171526113268558934119004209487;
