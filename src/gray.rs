// gray.rs      Pixel operations for gray pixel format.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use crate::Blend;
use pix::{Alpha, Channel, GammaMode, Gray, AssocGray, SepGray};

impl<C, A, G> Blend for SepGray<C, A, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    A: From<C>,
    G: GammaMode,
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
        let dst: AssocGray<C, A, G> = dst.into();
        let src: AssocGray<C, A, G> = src.into();

        AssocGray::<C, A, G>::over(dst, src).into()
    }
}

impl<C, A, G> Blend for AssocGray<C, A, G>
where
    C: Channel,
    A: Alpha<Chan = C>,
    A: From<C>,
    G: GammaMode,
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
        let v = src.value() + dst.value() * one_minus_src_a;

        Gray::with_alpha(v, a)
    }
}
