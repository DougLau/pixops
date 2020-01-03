// rgb.rs       Pixel operations for RGB pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::lerp::Lerp;
use crate::Blend;
use pix::{Alpha, Channel, Rgb, Format, Associated, GammaMode2};

impl<C, A, G> Blend for Rgb<C, A, Associated, G>
where
    C: Channel + Lerp,
    A: Alpha<Chan = C>,
    A: From<C>,
    G: GammaMode2,
{
    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B: Blend>(dst: &mut [Self], src: &[B], clr: Self)
    where
        Self: From<B>,
    {
        for (bot, top) in dst.iter_mut().zip(src) {
            let s = clr * Self::from(*top);
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        let one_minus_src_a = Self::Chan::MAX - src.alpha().value();
        let a = src.alpha().value() + dst.alpha().value() * one_minus_src_a;
        let r = src.red() + dst.red() * one_minus_src_a;
        let g = src.green() + dst.green() * one_minus_src_a;
        let b = src.blue() + dst.blue() * one_minus_src_a;

        Rgb::with_alpha(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::Blend;
    use pix::{Ch8, Translucent, Associated, Opaque, Srgb};

    #[test]
    fn rgba8_transparent() {
        // Test if transparent blending works.
        let t = pix::Rgb::<Ch8, Translucent<Ch8>, Associated, Srgb>::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::Rgb::<Ch8, Translucent<Ch8>, Associated, Srgb>::with_alpha(20, 40, 80, 160);

        let r1 = Blend::over(t, p);
        let r2 = Blend::over(p, t);

        assert_eq!(r1, p);
        assert_eq!(r1, r2);
    }

    #[test]
    fn transparent_over_white() {
        let t = pix::Rgb::<Ch8, Translucent<Ch8>, Associated, Srgb>::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::Rgb::<Ch8, Opaque<_>, Associated, Srgb>::new(0xFF, 0xFF, 0xFF).into();

        let r = Blend::over(p, t);

        assert_eq!(r, p);
    }
}
