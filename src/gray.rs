// gray.rs      Pixel operations for gray pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
// Copyright (c) 2020  Jeron Aldaron Lau
//
use crate::Blend;
use pix::{
    Alpha, AlphaMode, AssocGray, Channel, Format, GammaMode, Gray, Translucent,
};

impl<C, A, M, G> Blend for Gray<C, A, M, G>
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
        let clr: AssocGray<H, Translucent<H>, G> = clr.convert();

        for (bot, top) in dst.iter_mut().zip(src) {
            // Apply mask color to source raster.
            let src: AssocGray<H, Translucent<H>, G> = top.convert();
            let src = clr * src;

            // Over Operation
            *bot = Self::over(*bot, src);
        }
    }

    /// Blend pixel on top of another, using "over".
    fn over<B, H>(dst: Self, src: B) -> Self
    where
        B: Format<Chan = H>,
        Self::Chan: From<H>,
        H: Channel,
        H: From<C>,
    {
        let dst: AssocGray<H, Translucent<H>, G> = dst.convert();
        let src: AssocGray<H, Translucent<H>, G> = src.convert();

        let one_minus_src_a = H::MAX - src.alpha().value();
        let a = src.alpha().value() + dst.alpha().value() * one_minus_src_a;
        let v = src.value() + dst.value() * one_minus_src_a;

        AssocGray::<C, Translucent<C>, G>::with_alpha(v, a).convert()
    }
}
