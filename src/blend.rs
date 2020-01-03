// blend.rs     Pixel blend ops.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use pix::{Format};

/// Pixel format which can be blended.
pub trait Blend: Format {
    /// Blend pixels with `over` operation.
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_slice<B: Blend>(dst: &mut [Self], src: &[B], clr: Self)
    where
        Self: From<B>,
    {
        Self::over_fallback(dst, src, clr);
    }

    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B: Blend>(dst: &mut [Self], src: &[B], clr: Self)
    where
        Self: From<B>;

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self;
}
