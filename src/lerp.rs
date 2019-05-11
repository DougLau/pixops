// lerp.rs      Linear interpolation.
//
// Copyright (c) 2018-2019  Douglas P Lau
//
use pix::{Alpha, Channel, Ch8, Ch16, Ch32, Opaque, Translucent};

// NOTE: Lerp euqation is: (1 - t) * v0 + t * v1
//          Equivalent to: v0 + t * (v1 - v0)
pub trait Lerp {

    /// Linear interpolation
    fn lerp(self, rhs: Self, t: Self) -> Self;
}

impl Lerp for Ch8 {

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        let v0: i32 = u8::from(self).into();
        let v1: i32 = u8::from(rhs).into();
        let r = v0 + scale_i32(u8::from(t), v1 - v0);
        Ch8::new(r as u8)
    }
}

/// Scale an i32 value by a u8 (for lerp)
#[inline]
fn scale_i32(t: u8, v: i32) -> i32 {
    let c = v * i32::from(t);
    // cheap alternative to divide by 255
    ((c + 1) + (c >> 8)) >> 8
}

impl Lerp for Ch16 {

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        let v0: i64 = u16::from(self).into();
        let v1: i64 = u16::from(rhs).into();
        let r = v0 + scale_i64(u16::from(t), v1 - v0);
        Ch16::new(r as u16)
    }
}

/// Scale an i64 value by a u16 (for lerp)
#[inline]
fn scale_i64(t: u16, v: i64) -> i64 {
    let c = v * i64::from(t);
    // cheap alternative to divide by 65535
    ((c + 1) + (c >> 16)) >> 16
}

impl Lerp for Ch32 {

    /// Linear interpolation
    #[inline]
    fn lerp(self, rhs: Self, t: Self) -> Self {
        let v0 = f32::from(self);
        let v1 = f32::from(rhs);
        let r = v0 + f32::from(t) * (v1 - v0);
        Ch32::new(r)
    }
}

pub trait LerpAlpha: Alpha {

    /// Linear interpolation
    fn lerp(self, rhs: Self::Chan) -> Self;
}

impl<C: Channel + Lerp> LerpAlpha for Opaque<C> {

    /// Linear interpolation
    fn lerp(self, _rhs: C) -> Self {
        Opaque::default()
    }
}

impl<C: Channel + Lerp> LerpAlpha for Translucent<C> {

    /// Linear interpolation
    fn lerp(self, rhs: C) -> Self {
        Self::new(self.value().lerp(rhs, rhs))
    }
}
