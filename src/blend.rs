// blend.rs     Pixel blend ops.
//
// Copyright (c) 2018-2019  Douglas P Lau
// Copyright (c) 2020  Jeron Aldaron Lau
//
use pix::{Channel, Format};

/// Pixel format which can be blended.
pub trait Blend: Format {
    /// Blend pixels with `over` operation.
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_slice<B, H>(dst: &mut [Self], src: &[B], clr: Self)
    where
        B: Format<Chan = H>,
        Self::Chan: From<H>,
        H: Channel,
        H: From<Self::Chan>,
    {
        Self::over_fallback(dst, src, clr);
    }

    /// Blend pixels with `over` operation (slow fallback).
    ///
    /// * `dst` Destination pixels.
    /// * `src` Source pixels.
    /// * `clr` Mask color.
    fn over_fallback<B, H>(dst: &mut [Self], src: &[B], clr: Self)
    where
        B: Format<Chan = H>,
        Self::Chan: From<H>,
        H: Channel,
        H: From<Self::Chan>;

    /// Blend pixel on top of another, using "over".
    fn over<B, H>(dst: Self, src: B) -> Self
    where
        B: Format<Chan = H>,
        Self::Chan: From<H>,
        H: Channel,
        H: From<Self::Chan>;
}
