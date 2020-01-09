// mask.rs      Pixel operations for alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
// Copyright (c) 2020  Jeron Aldaron Lau
//
use crate::Blend;
use pix::{Alpha, Channel, Format, Mask};

impl<C, A> Blend for Mask<A>
where
    C: Channel,
    A: Alpha<Chan = C>,
    A: From<C>,
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
        for (bot, top) in dst.iter_mut().zip(src) {
            // Apply mask color to source raster.
            let src: Self = top.convert();
            let src = clr * src;

            // Over Operation
            *bot = Self::over::<Self, C>(*bot, src);
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
        let src: Self = src.convert();

        let one_minus_src_a = C::MAX - src.alpha().value();
        let a = src.alpha().value() + dst.alpha().value() * one_minus_src_a;

        Self::new(a)
    }
}
