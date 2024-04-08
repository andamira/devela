// devela::data::hash::pengy
//
//! Based on pengyhash v0.2 LICENSED as BSD-2.
//

/// Pengy hasher function.
#[must_use]
pub const fn hash_pengy(data: &[u8], seed: u32) -> u64 {
    let mut b: [u64; 4] = [0; 4];
    let mut s: [u64; 4] = [0, 0, 0, data.len() as u64];
    let mut remaining = data;

    while remaining.len() >= 32 {
        let mut i = 0;
        while i < 4 {
            let (_, rest) = remaining.split_at(i * 8);
            let (chunk, _) = rest.split_at(8);
            let mut byte_array: [u8; 8] = [0; 8];
            let mut j = 0;
            while j < chunk.len() {
                byte_array[j] = chunk[j];
                j += 1;
            }
            b[i] = u64::from_le_bytes(byte_array);

            i += 1;
        }
        let (_left, right) = remaining.split_at(32);
        remaining = right;

        s[0] = s[0].wrapping_add(s[1]).wrapping_add(b[3]);
        s[1] = s[0].wrapping_add(s[1].wrapping_shl(14) | s[1].wrapping_shr(50));
        s[2] = s[2].wrapping_add(s[3]).wrapping_add(b[2]);
        s[3] = s[2].wrapping_add(s[3].wrapping_shl(23) | s[3].wrapping_shr(41));
        s[0] = s[0].wrapping_add(s[3]).wrapping_add(b[1]);
        s[3] = s[0] ^ (s[3].wrapping_shl(16) | s[3].wrapping_shr(48));
        s[2] = s[2].wrapping_add(s[1]).wrapping_add(b[0]);
        s[1] = s[2] ^ (s[1].wrapping_shl(40) | s[1].wrapping_shr(24));
    }

    let mut tmp = [0u8; 32];
    let mut i = 0;
    while i < remaining.len() {
        if i < tmp.len() {
            tmp[i] = remaining[i];
        }
        i += 1;
    }

    let mut i = 0;
    while i < 4 {
        let (_, rest) = tmp.split_at(i * 8);
        let (chunk, _) = rest.split_at(8);
        let mut byte_array: [u8; 8] = [0; 8];
        let mut j = 0;
        while j < chunk.len() {
            byte_array[j] = chunk[j];
            j += 1;
        }
        b[i] = u64::from_le_bytes(byte_array);
        i += 1;
    }

    let mut _i = 0;
    while _i < 6 {
        s[0] = s[0].wrapping_add(s[1]).wrapping_add(b[3]);
        s[1] = s[0]
            .wrapping_add(s[1].wrapping_shl(14) | s[1].wrapping_shr(50))
            .wrapping_add(seed as u64);
        s[2] = s[2].wrapping_add(s[3]).wrapping_add(b[2]);
        s[3] = s[2].wrapping_add(s[3].wrapping_shl(23) | s[3].wrapping_shr(41));
        s[0] = s[0].wrapping_add(s[3]).wrapping_add(b[1]);
        s[3] = s[0] ^ (s[3].wrapping_shl(16) | s[3].wrapping_shr(48));
        s[2] = s[2].wrapping_add(s[1]).wrapping_add(b[0]);
        s[1] = s[2] ^ (s[1].wrapping_shl(40) | s[1].wrapping_shr(24));
        _i += 1;
    }
    s[0].wrapping_add(s[1])
        .wrapping_add(s[2])
        .wrapping_add(s[3])
}

// /// non-const, non-wrapping version, most similar to the original.
// pub fn pengyhash_orig(data: &[u8], seed: u32) -> u64 {
//     let mut b: [u64; 4] = [0; 4];
//     let mut s: [u64; 4] = [0, 0, 0, data.len() as u64];
//     let mut remaining = data;
//
//     while remaining.len() >= 32 {
//         for i in 0..4 {
//             b[i] = u64::from_le_bytes(remaining[i * 8..(i + 1) * 8].try_into().unwrap());
//         }
//         remaining = &remaining[32..];
//
//         s[0] += s[1] + b[3];
//         s[1] = s[0] + (s[1] << 14 | s[1] >> 50);
//         s[2] += s[3] + b[2];
//         s[3] = s[2] + (s[3] << 23 | s[3] >> 41);
//         s[0] += s[3] + b[1];
//         s[3] = s[0] ^ (s[3] << 16 | s[3] >> 48);
//         s[2] += s[1] + b[0];
//         s[1] = s[2] ^ (s[1] << 40 | s[1] >> 24);
//     }
//
//     let mut tmp = [0u8; 32];
//     tmp[..remaining.len()].copy_from_slice(remaining);
//     for i in 0..4 {
//         b[i] = u64::from_le_bytes(tmp[i * 8..(i + 1) * 8].try_into().unwrap());
//     }
//
//     for _ in 0..6 {
//         s[0] += s[1] + b[3];
//         s[1] = s[0] + (s[1] << 14 | s[1] >> 50) + seed as u64;
//         s[2] += s[3] + b[2];
//         s[3] = s[2] + (s[3] << 23 | s[3] >> 41);
//         s[0] += s[3] + b[1];
//         s[3] = s[0] ^ (s[3] << 16 | s[3] >> 48);
//         s[2] += s[1] + b[0];
//         s[1] = s[2] ^ (s[1] << 40 | s[1] >> 24);
//     }
//     s[0].wrapping_add(s[1])
//         .wrapping_add(s[2])
//         .wrapping_add(s[3])
// }
