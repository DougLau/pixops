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
            let s = clr * Self::from(*top);
            *bot = Blend::over(*bot, s);
        }
    }

    /// Blend pixel on top of another, using `over`.
    fn over(dst: Self, src: Self) -> Self {
        // Pre-multiply alpha destination
        let dst_a = dst.alpha().value();
        // let dst_r = dst.red();
        // let dst_g = dst.green();
        // let dst_b = dst.blue();

        // Pre-multiply alpha source
        let src_a = src.alpha().value();

        // SRC + DST * (1 - SRC_ALPHA)
        let one_minus_src_a = Self::Chan::MAX - src.alpha().value();
        let a = src.alpha().value() + dst.alpha().value() * one_minus_src_a;
        let r = src.red() * src_a + dst.red() * one_minus_src_a;
        let g = src.green() * src_a + dst.green() * one_minus_src_a;
        let b = src.blue() * src_a + dst.blue() * one_minus_src_a;

  //      let r = (dst.red() * dst_a).lerp(src.red() * src_a, src_a);
  //      let g = (dst.green() * dst_a).lerp(src.green() * src_a,  src_a);
  //      let b = (dst.blue() * dst_a).lerp(src.blue() * src_a, src_a);

        // Post-divide alpha destination
 //       let r = r / a;
   //     let g = g / a;
     //   let b = b / a;

//        let one_minus_srca = Self::Chan::MAX - src.alpha().value();
//        let alpha = src.alpha().value() + dst.alpha().value() * one_minus_srca;
//        let r = src.red() + dst.red() * one_minus_srca;
//        let g = src.green() + dst.green() * one_minus_srca;
//        let b = src.blue() + dst.blue() * one_minus_srca;

//        let a = src.alpha().value();
//        let r = dst.red().lerp(src.red(), src.alpha().value());
//        let g = dst.green().lerp(src.green(), src.alpha().value());
//        let b = dst.blue().lerp(src.blue(), src.alpha().value());
        Rgb::with_alpha(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::Blend;

    #[test]
    fn alpha() {
        // Clear destination
        let mut dst = [pix::Rgba8::with_alpha(0, 0, 0, 0)];
        let src = [pix::Rgba8::with_alpha(20, 40, 80, 0xFF)];
        super::Rgb::over_fallback(&mut dst, &src, pix::Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0xFF));
        assert_eq!(dst[0], pix::Rgba8::with_alpha(20, 40, 80, 0xFF));

        // Clear destination
        let mut dst = [pix::Rgba8::with_alpha(0, 0, 0, 0)];
        let src = [pix::Rgba8::with_alpha(20, 40, 80, 0xFF)];
        super::Rgb::over_fallback(&mut dst, &src, pix::Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0x80));
        assert_eq!(dst[0], pix::Rgba8::with_alpha(20, 40, 80, 0x80));

        // Clear destination
        let mut dst = [pix::Rgba8::with_alpha(0, 0, 0, 0)];
        let src = [pix::Rgba8::with_alpha(20, 40, 80, 0xFF)];
        super::Rgb::over_fallback(&mut dst, &src, pix::Rgba8::with_alpha(0xFF, 0xFF, 0xFF, 0x40));
        assert_eq!(dst[0], pix::Rgba8::with_alpha(20, 40, 80, 0x40));
    }

    #[test]
    fn rgba8_transparent() {
        // Test if transparent blending works.
        let t = pix::Rgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::Rgba8::with_alpha(20, 40, 80, 160);

        let r1 = Blend::over(t, p);
        let r2 = Blend::over(p, t);

        assert_eq!(r1, p);
        assert_eq!(r1, r2);
    }

    #[test]
    fn transparent_over_white() {
        let t = pix::Rgba8::with_alpha(0x00, 0x00, 0x00, 0x00);
        let p = pix::Rgb8::new(0xFF, 0xFF, 0xFF).into();

        let r = Blend::over(p, t);

        assert_eq!(r, p);
    }
}
