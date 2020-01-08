// mask.rs      Pixel operations for alpha mask pixel format.
//
// Copyright (c) 2019  Douglas P Lau
//
use crate::Blend;
use pix::{Alpha, Channel, Mask};

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

        Mask::new(a)
    }
}
