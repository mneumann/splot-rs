use image::{ImageBuffer, Pixel};
use std::ops::Range;

pub fn splot<P>(
    f: &impl Fn(f32, f32) -> f32,
    xrange: Range<f32>,
    yrange: Range<f32>,
    color_map: &impl Fn(f32) -> P,
    resolution_x: u32,
    resolution_y: u32,
) -> ImageBuffer<P, Vec<P::Subpixel>>
where
    P: Pixel + 'static,
{
    let width = (xrange.end - xrange.start).abs();
    let height = (yrange.end - yrange.start).abs();
    let dx = width / resolution_x as f32;
    let dy = height / resolution_y as f32;

    ImageBuffer::from_fn(resolution_x, resolution_y, |px: u32, py: u32| {
        let x = xrange.start + (px as f32 * dx);
        let y = yrange.start + (py as f32 * dy);
        let z = f(x, y);
        color_map(z)
    })
}

pub fn splot_rgb(
    f: &impl Fn(f32, f32) -> f32,
    xrange: Range<f32>,
    yrange: Range<f32>,
    color_map: &impl Fn(f32) -> image::Rgb<u8>,
    resolution_x: u32,
    resolution_y: u32,
    filename: &str,
) {
    splot(f, xrange, yrange, color_map, resolution_x, resolution_y)
        .save(filename)
        .unwrap();
}

pub fn splot_gray(
    f: &impl Fn(f32, f32) -> f32,
    xrange: Range<f32>,
    yrange: Range<f32>,
    color_map: &impl Fn(f32) -> image::Luma<u8>,
    resolution_x: u32,
    resolution_y: u32,
    filename: &str,
) {
    splot(f, xrange, yrange, color_map, resolution_x, resolution_y)
        .save(filename)
        .unwrap();
}

#[test]
fn test_image_save() {
    let f = |x: f32, y: f32| x.sin() * y.sin();
    let color_map = |z: f32| image::Rgb {
        data: [(z.max(0.0).min(1.0) * 255.0) as u8, 0, 0],
    };
    let gray_map = |z: f32| image::Luma {
        data: [(z.max(0.0).min(1.0) * 255.0) as u8],
    };

    splot_rgb(
        &f,
        -10.0..10.0,
        -10.0..10.0,
        &color_map,
        1024,
        1024,
        "color.png",
    );
    splot_gray(&f, -1.0..1.0, -1.0..1.0, &gray_map, 128, 128, "gray.png");
}
