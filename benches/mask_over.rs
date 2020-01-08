#[macro_use]
extern crate criterion;

use criterion::Criterion;
use pix::*;
use pixops::*;

fn mask_over_gray(c: &mut Criterion, sz: u32) {
    let s = format!("mask_over_gray_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = RasterBuilder::<SepSGrayAlpha8>::new().with_clear(sz, sz);
        let mut m = RasterBuilder::<Mask8>::new().with_clear(sz, sz);
        let c = Gray8::from(100);
        m.set_pixel(0, 0, 255);
        m.set_pixel(sz - 1, sz - 1, 128);
        b.iter(|| raster_over(&mut r, &m, c, 0, 0))
    });
}

fn mask_over_gray_16(c: &mut Criterion) {
    mask_over_gray(c, 16);
}

fn mask_over_gray_256(c: &mut Criterion) {
    mask_over_gray(c, 256);
}

fn mask_over_gray_512(c: &mut Criterion) {
    mask_over_gray(c, 512);
}

fn mask_over_rgba(c: &mut Criterion, sz: u32) {
    let s = format!("mask_over_rgba_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = RasterBuilder::<SepSRgba8>::new().with_clear(sz, sz);
        let mut m = RasterBuilder::<Mask8>::new().with_clear(sz, sz);
        let rgba: SepSRgba8 = Rgb::with_alpha(100, 50, 150, 255);
        m.set_pixel(0, 0, 255);
        m.set_pixel(sz - 1, sz - 1, 128);
        b.iter(|| raster_over(&mut r, &m, rgba, 0, 0))
    });
}

fn mask_over_rgba_16(c: &mut Criterion) {
    mask_over_rgba(c, 16);
}

fn mask_over_rgba_256(c: &mut Criterion) {
    mask_over_rgba(c, 256);
}

fn mask_over_rgba_512(c: &mut Criterion) {
    mask_over_rgba(c, 512);
}

criterion_group!(
    benches,
    mask_over_gray_16,
    mask_over_gray_256,
    mask_over_gray_512,
    mask_over_rgba_16,
    mask_over_rgba_256,
    mask_over_rgba_512
);

criterion_main!(benches);
