use crate::{Color, ColorMap, BLACK, WHITE};
use std::ops::RangeInclusive;

pub struct ColorPalette {
    colors: Vec<Color>,
}

impl ColorPalette {
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    pub fn grayscale(n: usize) -> Self {
        Self::linear_gradient(n, BLACK, WHITE)
    }

    pub fn linear_gradient(n: usize, start_color: Color, end_color: Color) -> Self {
        assert!(n > 1);
        let mut palette = Self::new();
        let d0 =
            (f32::from(end_color.rgb[0]) - f32::from(start_color.rgb[0])).abs() / (n - 1) as f32;
        let d1 =
            (f32::from(end_color.rgb[1]) - f32::from(start_color.rgb[1])).abs() / (n - 1) as f32;
        let d2 =
            (f32::from(end_color.rgb[2]) - f32::from(start_color.rgb[2])).abs() / (n - 1) as f32;

        for i in 0..n {
            palette.add_color(
                (
                    (start_color.rgb[0] as f32 + (d0 * i as f32))
                        .min(255.0)
                        .max(0.0) as u8,
                    (start_color.rgb[1] as f32 + (d1 * i as f32))
                        .min(255.0)
                        .max(0.0) as u8,
                    (start_color.rgb[2] as f32 + (d2 * i as f32))
                        .min(255.0)
                        .max(0.0) as u8,
                )
                    .into(),
            );
        }
        palette
    }

    pub fn add_color(&mut self, color: Color) {
        self.colors.push(color.into());
    }
}

impl ColorMap for ColorPalette {
    fn map(&self, z: f32, zrange: &RangeInclusive<f32>) -> Color {
        assert!(!self.colors.is_empty());
        let depth = (zrange.end() - zrange.start()).abs();
        let dz = depth / self.colors.len() as f32;
        let dist = z - zrange.start();
        assert!(dist >= 0.0);
        let color_idx = ((dist / dz) as usize).min(self.colors.len() - 1);
        self.colors[color_idx]
    }
}
