// rgb.rs       Pixel operations for RGB pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::lerp::Lerp;
use crate::Blend;
use pix::{Alpha, Channel, Rgb};

impl<C, A> Blend for Rgb<C, A>
where
    C: Channel + Lerp,
    A: Alpha<Chan = C>,
    A: From<C>,
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
            let s = clr * Into::<Self>::into(*top);
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        let alpha = src.alpha().value();
        let r = dst.red().lerp(src.red(), alpha);
        let g = dst.green().lerp(src.green(), alpha);
        let b = dst.blue().lerp(src.blue(), alpha);
        Rgb::with_alpha(r, g, b, alpha)
    }
}
