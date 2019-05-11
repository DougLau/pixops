// rgb.rs       Pixel operations for RGB pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use pix::{Alpha, Channel, Rgb};
use crate::Blend;
use crate::lerp::Lerp;

impl<C, A> Blend for Rgb<C, A>
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
            // FIXME: combine with clr somehow!
            let s = Into::<Self>::into(*top);
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        let a = src.alpha().value();
        let r = dst.red().lerp(src.red(), a);
        let g = dst.green().lerp(src.green(), a);
        let b = dst.blue().lerp(src.blue(), a);
        let alpha = dst.alpha().value().lerp(a, a);
        Rgb::with_alpha(r, g, b, alpha)
    }
}
