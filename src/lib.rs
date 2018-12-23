use image::{ImageBuffer, Rgb, RgbImage};
use std::ops::{Range, RangeInclusive};

pub trait ColorMap {
    fn map(&self, z: f32, zrange: &RangeInclusive<f32>) -> Rgb<u8>;
}

pub struct ColorPalette {
    colors: Vec<Rgb<u8>>,
}

impl ColorPalette {
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    pub fn grayscale(n: usize) -> Self {
        assert!(n > 1);
        let mut palette = Self::new();
        for i in 0..n {
            let gray = (i as f32) / (n - 1) as f32;
            palette.add_rgb(gray, gray, gray);
        }
        palette
    }

    pub fn linear_gradient(n: usize, start: Rgb<u8>, end: Rgb<u8>) -> Self {
        assert!(n > 1);
        let mut palette = Self::new();
        let d0 = (f32::from(end[0]) - f32::from(start[0])).abs() / (n - 1) as f32;
        let d1 = (f32::from(end[1]) - f32::from(start[1])).abs() / (n - 1) as f32;
        let d2 = (f32::from(end[2]) - f32::from(start[2])).abs() / (n - 1) as f32;

        for i in 0..n {
            palette.add_color(Rgb {
                data: [
                    (start[0] + (d0 * i as f32) as u8).min(255),
                    (start[1] + (d1 * i as f32) as u8).min(255),
                    (start[2] + (d2 * i as f32) as u8).min(255),
                ],
            });
        }
        palette
    }

    pub fn add_rgb(&mut self, r: f32, g: f32, b: f32) {
        assert!(r >= 0.0 && r <= 1.0);
        assert!(g >= 0.0 && g <= 1.0);
        assert!(b >= 0.0 && b <= 1.0);
        self.add_color(Rgb {
            data: [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8],
        });
    }

    pub fn add_color(&mut self, color: Rgb<u8>) {
        self.colors.push(color);
    }
}

impl ColorMap for ColorPalette {
    fn map(&self, z: f32, zrange: &RangeInclusive<f32>) -> Rgb<u8> {
        assert!(!self.colors.is_empty());
        let depth = (zrange.end() - zrange.start()).abs();
        let dz = depth / self.colors.len() as f32;
        let dist = z - zrange.start();
        assert!(dist >= 0.0);
        let color_idx = ((dist / dz) as usize).min(self.colors.len() - 1);
        self.colors[color_idx]
    }
}

pub struct SurfaceFn<F>
where
    F: Fn(f32, f32) -> f32,
{
    pub f: F,
    pub xrange: Range<f32>,
    pub yrange: Range<f32>,
}

impl<F> SurfaceFn<F>
where
    F: Fn(f32, f32) -> f32,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            xrange: -1.0..1.0,
            yrange: -1.0..1.0,
        }
    }

    pub fn width(&self) -> f32 {
        (self.xrange.end - self.xrange.start).abs()
    }

    pub fn height(&self) -> f32 {
        (self.yrange.end - self.yrange.start).abs()
    }

    pub fn with_xrange(self, xrange: Range<f32>) -> Self {
        Self { xrange, ..self }
    }

    pub fn with_yrange(self, yrange: Range<f32>) -> Self {
        Self { yrange, ..self }
    }

    pub fn sample_zrange(&self, resolution_x: u32, resolution_y: u32) -> RangeInclusive<f32> {
        assert!(resolution_x > 0 && resolution_y > 0);

        let dx = self.width() / resolution_x as f32;
        let dy = self.height() / resolution_y as f32;
        // Rasterized version of f
        let f_r = |px: u32, py: u32| {
            let x = self.xrange.start + (px as f32 * dx);
            let y = self.yrange.start + (py as f32 * dy);
            (self.f)(x, y)
        };

        let mut min_z = f_r(0, 0);
        let mut max_z = min_z;

        for x in 0..resolution_x {
            for y in 0..resolution_y {
                let z = f_r(x, y);
                min_z = min_z.min(z);
                max_z = max_z.max(z);
            }
        }

        (min_z..=max_z)
    }

    pub fn plot(
        &self,
        color_map: &impl ColorMap,
        resolution_x: u32,
        resolution_y: u32,
    ) -> RgbImage {
        let dx = self.width() / resolution_x as f32;
        let dy = self.height() / resolution_y as f32;
        // Rasterized version of f
        let f_r = |px: u32, py: u32| {
            let x = self.xrange.start + (px as f32 * dx);
            let y = self.yrange.start + (py as f32 * dy);
            (self.f)(x, y)
        };

        let zrange = self.sample_zrange(resolution_x, resolution_y);

        ImageBuffer::from_fn(resolution_x, resolution_y, |px: u32, py: u32| {
            color_map.map(f_r(px, py), &zrange)
        })
    }
}

#[test]
fn test_image_save() {
    let surface = SurfaceFn::new(|x: f32, y: f32| x.sin() * y.sin())
        .with_xrange(-10.0..10.0)
        .with_yrange(-10.0..10.0);

    let redish =
        ColorPalette::linear_gradient(256, Rgb { data: [0, 0, 0] }, Rgb { data: [255, 0, 0] });

    surface.plot(&redish, 1024, 1024).save("color.png").unwrap();
}
