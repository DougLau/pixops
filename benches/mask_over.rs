#[macro_use]
extern crate criterion;
extern crate pix;
extern crate pixops;

use pix::*;
use pixops::*;
use criterion::Criterion;

fn mask_over_gray(c: &mut Criterion, sz: u32) {
    let s = format!("mask_over_gray_{}", sz);
    c.bench_function(&s, move |b| {
        let mut r = Raster::<Gray8>::new(sz, sz);
        let mut m = Raster::<Mask8>::new(sz, sz);
        let c = Gray8::from(100);
        m.set_pixel(0, 0, 255);
        m.set_pixel(sz - 1, sz - 1, 128);
        b.iter(|| Raster::<Gray8>::mask_over(&mut r, &m, 0, 0, c))
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
        let mut r = Raster::<Rgba8>::new(sz, sz);
        let mut m = Raster::<Mask8>::new(sz, sz);
        let rgba: Rgba8 = Rgb::new(100, 50, 150, 255);
        m.set_pixel(0, 0, 255);
        m.set_pixel(sz - 1, sz - 1, 128);
        b.iter(|| Raster::<Rgba8>::mask_over(&mut r, &m, 0, 0, rgba))
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

criterion_group!(benches, mask_over_gray_16, mask_over_gray_256,
    mask_over_gray_512, mask_over_rgba_16, mask_over_rgba_256,
    mask_over_rgba_512);

criterion_main!(benches);
