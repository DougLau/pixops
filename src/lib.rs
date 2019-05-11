// lib.rs      Pixops crate.
//
// Copyright (c) 2019  Douglas P Lau
//
//! Pixel operations crate.
//!
mod blend;
mod gray;
mod lerp;
mod mask;
mod raster;
mod rgb;

pub use crate::blend::Blend;
pub use crate::raster::raster_over;
