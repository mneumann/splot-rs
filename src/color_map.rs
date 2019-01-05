use crate::Color;
use std::ops::RangeInclusive;

pub trait ColorMap {
    fn map(&self, z: f32, zrange: &RangeInclusive<f32>) -> Color;
}
