// rgb.rs       Pixel operations for RGB pixel format.
//
// Copyright (c) 2019  Douglas P Lau
// Copyright (c) 2020  Jeron Aldaron Lau
//
use crate::Blend;
use pix::{
    Alpha, AlphaMode, AssocRgb, Channel, Format, GammaMode, Rgb, Translucent,
};

impl<C, A, M, G> Blend for Rgb<C, A, M, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    A: From<C>,
    M: AlphaMode,
    G: GammaMode,
{
    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B, H>(dst: &mut [Self], src: &[B], clr: Self)
    where
        B: Format<Chan = H>,
        C: From<H>,
        H: Channel,
        H: From<C>,
    {
        // Over operation requires alpha is Associated, translucency needed for
        // blending.
        let clr: AssocRgb<H, Translucent<H>, G> = clr.convert();

        for (bot, top) in dst.iter_mut().zip(src) {
            // Apply mask color to source raster.
            let src: AssocRgb<H, Translucent<H>, G> = top.convert();
            let src = clr * src;

            // Over Operation
            *bot = Self::over(*bot, src);
        }
    }

    /// Blend pixel on top of another, using "over".
    fn over<B, H>(dst: Self, src: B) -> Self
    where
        B: Format<Chan = H>,
        C: From<H>,
        H: Channel,
        H: From<C>,
    {
        let dst: AssocRgb<H, Translucent<H>, G> = dst.convert();
        let src: AssocRgb<H, Translucent<H>, G> = src.convert();

        let one_minus_src_a = H::MAX - src.alpha().value();
        let a = src.alpha().value() + dst.alpha().value() * one_minus_src_a;
        let r = src.red() + dst.red() * one_minus_src_a;
        let g = src.green() + dst.green() * one_minus_src_a;
        let b = src.blue() + dst.blue() * one_minus_src_a;

        AssocRgb::<C, Translucent<C>, G>::with_alpha(r, g, b, a).convert()
    }
}

#[cfg(test)]
mod tests {
    use super::Blend;

    #[test]
    fn rgba8_transparent() {
        // Test if transparent blending works.
        let t = pix::AssocSRgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::AssocSRgba8::with_alpha(20, 40, 80, 160);

        let r1 = Blend::over(t, p);
        let r2 = Blend::over(p, t);

        assert_eq!(r1, p);
        assert_eq!(r1, r2);
    }

    #[test]
    fn transparent_over_white() {
        let t = pix::AssocSRgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::AssocSRgba8::new(0xFF, 0xFF, 0xFF);

        let r = Blend::over(p, t);

        assert_eq!(r, p);
    }
}
