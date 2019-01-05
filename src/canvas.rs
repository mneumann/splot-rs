use crate::{Color, ColorMap, Surface2};
use image::RgbImage;
use std::ops::RangeInclusive;

/// Represents a pixel canvas
#[derive(Debug)]
pub struct Canvas {
    image: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width > 0 && height > 0);
        Self {
            image: RgbImage::new(width, height),
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    fn xy_visible(&self, x: u32, y: u32) -> bool {
        self.x_visible(x) && self.y_visible(y)
    }

    fn x_visible(&self, x: u32) -> bool {
        x < self.image.width()
    }

    fn y_visible(&self, y: u32) -> bool {
        y < self.image.height()
    }

    fn clip_x(&self, x: u32) -> u32 {
        x.min(self.image.width() - 1)
    }

    fn clip_y(&self, y: u32) -> u32 {
        y.min(self.image.height() - 1)
    }

    /// Draws a single pixel.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        if self.xy_visible(x, y) {
            self.image.put_pixel(x, y, color.rgb);
        }
    }

    /// Draws a horizontal line.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_hline(&mut self, x1: u32, y1: u32, w: u32, color: Color) {
        if self.y_visible(y1) {
            let x2 = self.clip_x(x1 + w);
            for xi in x1..x2 {
                self.image.put_pixel(xi, y1, color.rgb);
            }
        }
    }

    pub fn draw_hline2(&mut self, x1: i32, y1: i32, w: u32, color: Color) {
        if y1 >= 0 {
            let x2 = x1 + w as i32;
            if x2 >= 0 {
                if x1 >= 0 {
                    self.draw_hline(x1 as u32, y1 as u32, w, color);
                } else {
                    self.draw_hline(0, y1 as u32, ((w as i32) + x1).abs() as u32, color);
                }
            }
        }
    }

    /// Draws a vertial line.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_vline(&mut self, x1: u32, y1: u32, h: u32, color: Color) {
        if self.x_visible(x1) {
            let y2 = self.clip_y(y1 + h);
            for yi in y1..=y2 {
                self.image.put_pixel(x1, yi, color.rgb);
            }
        }
    }

    /// Draws a rectangle.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_rect(&mut self, x1: u32, y1: u32, w: u32, h: u32, color: Color) {
        self.draw_hline(x1, y1, w, color);
        self.draw_hline(x1, y1 + h, w, color);
        self.draw_vline(x1, y1, h, color);
        self.draw_vline(x1 + w, y1, h, color);
    }

    /// Draws a rectangle.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_square(&mut self, x1: u32, y1: u32, d: u32, color: Color) {
        self.draw_rect(x1, y1, d, d, color);
    }

    /// Draws a filled circle.
    ///
    /// Does not panic if coordinates are out of range.
    pub fn draw_filled_circle(&mut self, x1: u32, y1: u32, r: u32, color: Color) {
        self.draw_hline2(x1 as i32 - r as i32, y1 as i32, 2 * r, color);

        for i in 1..=r {
            let w = ((r as f32).powi(2) - (i as f32).powi(2)).sqrt() as u32;
            self.draw_hline2(x1 as i32 - w as i32, y1 as i32 - i as i32, 2 * w, color);
            self.draw_hline2(x1 as i32 - w as i32, y1 as i32 + i as i32, 2 * w, color);
        }
    }

    pub fn make_transformation(
        &self,
        xrange: &RangeInclusive<f32>,
        yrange: &RangeInclusive<f32>,
    ) -> RasterTransformation {
        let dx = (*xrange.end() - *xrange.start()).abs() / self.width() as f32;
        let dy = (*yrange.end() - *yrange.start()).abs() / self.height() as f32;

        RasterTransformation {
            width: self.width(),
            height: self.height(),
            xrange: xrange.clone(),
            yrange: yrange.clone(),
            dx,
            dy,
        }
    }

    pub fn sample_surface2_zrange<F>(&self, surface: &Surface2<F>) -> RangeInclusive<f32>
    where
        F: Fn((f32, f32)) -> f32,
    {
        let transformation = self.make_transformation(&surface.xrange, &surface.yrange);
        let f_r = |px: u32, py: u32| (surface.f)(transformation.raster_to_image(px, py));

        let mut min_z = f_r(0, 0);
        let mut max_z = min_z;

        for x in 0..self.width() {
            for y in 0..self.height() {
                let z = f_r(x, y);
                min_z = min_z.min(z);
                max_z = max_z.max(z);
            }
        }

        (min_z..=max_z)
    }

    pub fn splot<F>(&mut self, surface: &Surface2<F>, color_map: &impl ColorMap) -> &mut Self
    where
        F: Fn((f32, f32)) -> f32,
    {
        let transformation = self.make_transformation(&surface.xrange, &surface.yrange);
        let f_r = |px: u32, py: u32| {
            let (x, y) = transformation.raster_to_image(px, py);
            (surface.f)((x, y))
        };
        let zrange = self.sample_surface2_zrange(surface);

        for (px, py, pixel) in self.image.enumerate_pixels_mut() {
            *pixel = color_map.map(f_r(px, py), &zrange).into()
        }
        self
    }

    pub fn image(&self) -> &RgbImage {
        &self.image
    }
}

/// Map a (x,y) coordinate within xrange and yrange into the given raster coordinate system and
/// vice versa.
pub struct RasterTransformation {
    width: u32,
    height: u32,
    xrange: RangeInclusive<f32>,
    yrange: RangeInclusive<f32>,
    dx: f32,
    dy: f32,
}

impl RasterTransformation {
    pub fn raster_to_image(&self, rx: u32, ry: u32) -> (f32, f32) {
        debug_assert!(rx < self.width);
        debug_assert!(ry < self.height);
        let x = *self.xrange.start() + (rx as f32 * self.dx);
        let y = *self.yrange.start() + (ry as f32 * self.dy);
        debug_assert!(x >= *self.xrange.start() && x <= *self.xrange.end());
        debug_assert!(y >= *self.yrange.start() && y <= *self.yrange.end());
        (x, y)
    }

    pub fn image_to_raster(&self, x: f32, y: f32) -> (u32, u32) {
        /*
        debug_assert!(x >= self.xrange.start);
        debug_assert!(x <= self.xrange.end);
        debug_assert!(y >= self.yrange.start);
        debug_assert!(y <= self.yrange.end);
        */
        let x = x.min(*self.xrange.end()).max(*self.xrange.start());
        let y = y.min(*self.yrange.end()).max(*self.yrange.start());
        let rx = (((x - *self.xrange.start()) / self.dx) as u32).min(self.width - 1);
        let ry = (((y - *self.yrange.start()) / self.dy) as u32).min(self.height - 1);
        debug_assert!(rx < self.width);
        debug_assert!(ry < self.height);
        (rx, ry)
    }
}
