pub mod canvas;
pub mod color;
pub mod color_map;
pub mod color_palette;
pub mod surface2;

pub use crate::canvas::Canvas;
pub use crate::color::*;
pub use crate::color_map::ColorMap;
pub use crate::color_palette::ColorPalette;
pub use crate::surface2::*;

#[test]
fn test_image_save() {
    let surface = Surface2::new(|(x, y): (f32, f32)| x.sin() * y.sin())
        .with_xrange(-10.0..=10.0)
        .with_yrange(-10.0..=10.0);

    let redish = ColorPalette::linear_gradient(256, Black, Red);
    let gray = ColorPalette::grayscale(256);

    let mut canvas = Canvas::new(1024, 1024);
    canvas.splot(&surface, &redish);
    canvas.draw_rect(50, 50, 100, 100, Green.into());
    canvas.draw_filled_circle(200, 200, 20, Blue.into());

    canvas.image().save("color.png").unwrap();

    Canvas::new(1024, 1024)
        .splot(&surface, &gray)
        .image()
        .save("gray.png")
        .unwrap();
}
