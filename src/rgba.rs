// rgb.rs       Rgb pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use pix::{Mask8, Rgba8};
use crate::Blend;

#[cfg(all(target_arch = "x86", feature = "simd"))]
use std::arch::x86::*;
#[cfg(all(target_arch = "x86_64", feature = "simd"))]
use std::arch::x86_64::*;

impl Blend for Rgba8 {
    /// Blend pixels with an alpha mask.
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over(dst: &mut [Self], mask: &[u8], clr: Self) {
        #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
              feature = "simd"))]
        {
            if is_x86_feature_detected!("ssse3") {
                let len = dst.len().min(mask.len());
                if len >= 4 {
                    unsafe { over_x86(dst, mask, clr) }
                }
                let ln = (len >> 2) << 2;
                if len > ln {
                    Self::mask_over_fallback(&mut dst[ln..], &mask[ln..], clr);
                }
                return;
            }
        }
        Blend::mask_over_fallback(dst, mask, clr);
    }

    /// Blend pixels with an alpha mask (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `mask` Alpha mask for compositing.
    /// * `src` Source color.
    fn mask_over_fallback(dst: &mut [Self], mask: &[u8], src: Self) {
        for (bot, m) in dst.iter_mut().zip(mask) {
            *bot = src.with_alpha_over(*bot, *m);
        }
    }
}

/// Composite a color with a mask.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn over_x86(pix: &mut [Rgb<Ch8>], mask: &[u8], clr: Rgb<Ch8>) {
    let len = pix.len().min(mask.len());
    // Truncate len to multiple of 4
    let len = (len >> 2) << 2;
    let clr = _mm_set1_epi32(clr.into());
    let src = mask.as_ptr();
    let dst = pix.as_mut_ptr();
    // 4 pixels at a time
    for i in (0..len).step_by(4) {
        let off = i as isize;
        let dst = dst.offset(off) as *mut __m128i;
        let src = src.offset(off) as *const i32;
        // get 4 alpha values from src,
        // then shuffle: xxxxxxxxxxxx3210 => 3333222211110000
        let alpha = swizzle_mask_x86(_mm_set1_epi32(*src));
        // get RGBA values from dst
        let bot = _mm_loadu_si128(dst);
        // compose top over bot
        let out = over_alpha_u8x16_x86(clr, bot, alpha);
        // store blended pixels
        _mm_storeu_si128(dst, out);
    }
}

/// Swizzle alpha mask (xxxxxxxxxxxx3210 => 3333222211110000)
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn swizzle_mask_x86(v: __m128i) -> __m128i {
    _mm_shuffle_epi8(v, _mm_set_epi8(3, 3, 3, 3,
                                     2, 2, 2, 2,
                                     1, 1, 1, 1,
                                     0, 0, 0, 0))
}

/// Composite packed u8 values using `over`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn over_alpha_u8x16_x86(t: __m128i, b: __m128i, a: __m128i) -> __m128i {
    // Since alpha can range from 0 to 255 and (t - b) can range from -255 to
    // +255, we would need 17 bits to store the result of a multiplication.
    // Instead, shift alpha right by 1 bit (divide by 2).  Afterwards, we can
    // shift back by one less bit (in scale_i16_to_u8_x86).
    // For even lanes: b + alpha * (t - b)
    let t_even = _mm_unpacklo_epi8(t, _mm_setzero_si128());
    let b_even = _mm_unpacklo_epi8(b, _mm_setzero_si128());
    let a_even = _mm_unpacklo_epi8(a, _mm_setzero_si128());
    let a_even = _mm_srli_epi16(a_even, 1);
    let even = _mm_mullo_epi16(a_even, _mm_sub_epi16(t_even, b_even));
    let even = scale_i16_to_u8_x86(even);
    let even = _mm_add_epi16(b_even, even);
    // For odd lanes: b + alpha * (t - b)
    let t_odd = _mm_unpackhi_epi8(t, _mm_setzero_si128());
    let b_odd = _mm_unpackhi_epi8(b, _mm_setzero_si128());
    let a_odd = _mm_unpackhi_epi8(a, _mm_setzero_si128());
    let a_odd = _mm_srli_epi16(a_odd, 1);
    let odd = _mm_mullo_epi16(a_odd, _mm_sub_epi16(t_odd, b_odd));
    let odd = scale_i16_to_u8_x86(odd);
    let odd = _mm_add_epi16(b_odd, odd);
    _mm_packus_epi16(even, odd)
}

/// Scale i16 values (result of "u7" * "i9") into u8.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
          feature = "simd"))]
#[target_feature(enable = "ssse3")]
unsafe fn scale_i16_to_u8_x86(v: __m128i) -> __m128i {
    // To scale into a u8, we would normally divide by 255.  This is equivalent
    // to: ((v + 1) + (v >> 8)) >> 8
    // For the last right shift, we use 7 instead to simulate multiplying by
    // 2.  This is necessary because alpha was shifted right by 1 bit to allow
    // fitting 17 bits of data into epi16 lanes.
    _mm_srai_epi16(_mm_add_epi16(_mm_add_epi16(v,
                                               _mm_set1_epi16(1)),
                                 _mm_srai_epi16(v, 8)),
                   7)
}
