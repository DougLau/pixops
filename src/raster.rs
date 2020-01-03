// raster.rs    A 2D raster image.
//
// Copyright (c) 2017-2019  Douglas P Lau
//
use crate::Blend;
use pix::{Raster, AlphaMode};

/// Blend targets with `over` operation.
///
/// * `dst` Destination target.
/// * `src` Source target.
/// * `clr` Default blend color.
/// * `x` Left position of source on destination.
/// * `y` Top position of source on destination.
pub fn raster_over<A, B, C>(
    dst: &mut Raster<A>,
    src: &Raster<B>,
    clr: C,
    x: i32,
    y: i32,
) where
    A: Blend,
    B: Blend,
    C: Blend,
    A: From<B>,
    A: From<C>,
{
    assert_eq!(dst.alpha_mode(), AlphaMode::Associated);
    assert_eq!(src.alpha_mode(), AlphaMode::Associated);

    let clr = Into::<A>::into(clr);
    if x == 0
        && dst.width() == src.width()
        && y == 0
        && dst.height() == src.height()
    {
        A::over_slice(dst.as_slice_mut(), src.as_slice(), clr);
        return;
    }
    if x + (src.width() as i32) < 0 || x >= dst.width() as i32 {
        return; // positioned off left or right edge
    }
    if y + (src.height() as i32) < 0 || y >= dst.height() as i32 {
        return; // positioned off top or bottom edge
    }
    let mx = 0.max(-x) as usize;
    let my = 0.max(-y) as u32;
    let mw = src.width() as usize;
    let dx = 0.max(x) as usize;
    let dy = 0.max(y) as u32;
    let dw = dst.width() as usize;
    let h = (dst.height() - dy).min(src.height() - my);
    for yi in 0..h {
        let mut row = &mut dst.as_slice_row_mut(dy + yi)[dx..dw];
        let m = &src.as_slice_row(my + yi)[mx..mw];
        A::over_slice(&mut row, m, clr);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pix::*;
    #[test]
    fn raster_mask() {
        let mut r = RasterBuilder::<Rgba8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(3, 3);
        let mut m = RasterBuilder::<Mask8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(3, 3);
        m.set_pixel(0, 0, 0xFF);
        m.set_pixel(1, 1, 0x80);
        m.set_pixel(2, 2, 0x40);
        let c: Rgb8 = Rgb::new(0xFF, 0x80, 0x40);
        raster_over(&mut r, &m, c, 0, 0);
        #[rustfmt::skip]
        let v = [
            0xFF, 0x80, 0x40, 0xFF,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x80, 0x40, 0x20, 0x80,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x40, 0x20, 0x10, 0x40,
        ];
        let left = r.as_u8_slice();
        // NOTE: fallback version     SIMD version
        assert!(left[0] == 0xFF || left[0] == 0xFE);
        assert!(left[1] == 0x80 || left[1] == 0x7F);
        assert!(left[2] == 0x40 || left[2] == 0x3F);
        assert!(left[3] == 0xFF || left[3] == 0xFE);
        assert_eq!(&left[4..], &v[4..]);
    }
    #[test]
    fn smaller_mask() {
        let mut r = RasterBuilder::<Rgba8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(3, 3);
        let mut m = RasterBuilder::<Mask8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(2, 2);
        let c: Rgba8 = Rgb::with_alpha(0x40, 0x80, 0x60, 0x80);
        m.set_pixel(0, 0, 0xFF);
        m.set_pixel(1, 0, 0x80);
        m.set_pixel(0, 1, 0x40);
        m.set_pixel(1, 1, 0x20);
        raster_over(&mut r, &m, c, 1, 1);
        #[rustfmt::skip]
        let v = [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x40, 0x80, 0x60, 0x80,
            0x20, 0x40, 0x30, 0x40,

            0x00, 0x00, 0x00, 0x00,
            0x10, 0x20, 0x18, 0x20,
            0x08, 0x10, 0x0C, 0x10,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn top_left() {
        let mut r = RasterBuilder::<Rgba8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(3, 3);
        let m = RasterBuilder::<Mask8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_color(2, 2, Mask8::new(0xFF));
        let c = Rgb8::new(0x20, 0x40, 0x80);
        raster_over(&mut r, &m, c, -1, -1);
        #[rustfmt::skip]
        let v = [
            0x20, 0x40, 0x80, 0xFF,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
    #[test]
    fn bottom_right() {
        let mut r = RasterBuilder::<Rgba8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(3, 3);
        let mut m = RasterBuilder::<Mask8>::new()
            .alpha_mode(AlphaMode::Associated)
            .with_clear(2, 2);
        let c: Rgb8 = Rgb::new(0x20, 0x40, 0x80);
        m.set_pixel(0, 0, 0xFF);
        m.set_pixel(1, 0, 0xFF);
        m.set_pixel(0, 1, 0xFF);
        m.set_pixel(1, 1, 0xFF);
        raster_over(&mut r, &m, c, 2, 2);
        #[rustfmt::skip]
        let v = [
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,

            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x20, 0x40, 0x80, 0xFF,
        ];
        assert_eq!(r.as_u8_slice(), &v[..]);
    }
}
