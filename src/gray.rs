// gray.rs      Pixel operations for gray pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use pix::{Alpha, Channel, Gray};
use crate::Blend;
use crate::lerp::Lerp;

impl<C, A> Blend for Gray<C, A>
    where C: Channel + Lerp, A: Alpha<Chan=C>, A: From<C>
{

    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B: Blend>(dst: &mut [Self], src: &[B], _clr: Self)
        where Self: From<B>
    {
        for (bot, top) in dst.iter_mut().zip(src) {
            let s = Self::from(*top);
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        let a = src.alpha().value();
        let value = dst.value().lerp(src.value(), a);
        let alpha = dst.alpha().value().lerp(a, a);
        Gray::with_alpha(value, alpha)
    }
}