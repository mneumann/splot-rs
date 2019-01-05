use std::ops::RangeInclusive;

pub struct Surface2<F>
where
    F: Fn((f32, f32)) -> f32,
{
    pub f: F,
    pub xrange: RangeInclusive<f32>,
    pub yrange: RangeInclusive<f32>,
}

impl<F> Surface2<F>
where
    F: Fn((f32, f32)) -> f32,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            xrange: -1.0..=1.0,
            yrange: -1.0..=1.0,
        }
    }

    pub fn width(&self) -> f32 {
        (self.xrange.end() - self.xrange.start()).abs()
    }

    pub fn height(&self) -> f32 {
        (self.yrange.end() - self.yrange.start()).abs()
    }

    pub fn with_xrange(self, xrange: RangeInclusive<f32>) -> Self {
        Self { xrange, ..self }
    }

    pub fn with_yrange(self, yrange: RangeInclusive<f32>) -> Self {
        Self { yrange, ..self }
    }
}
