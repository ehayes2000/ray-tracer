use crate::Float;
use crate::math::Color;
use std::io::Write;

use crate::interval::Interval;

static INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

fn linear_to_gamma(component: Float) -> Float {
    if component > 0.0 {
        Float::sqrt(component)
    } else {
        component
    }
}

pub fn write_color(mut f: impl Write, color: &Color) -> Result<(), std::io::Error> {
    let (r, g, b) = to_8bit(color);
    writeln!(f, "{} {} {}", r, g, b)
}

pub fn to_8bit(color: &Color) -> (u8, u8, u8) {
    let r = linear_to_gamma(color.0);
    let g = linear_to_gamma(color.1);
    let b = linear_to_gamma(color.2);
    let r = (256.0 * INTENSITY.clamp(r)) as u8;
    let g = (256.0 * INTENSITY.clamp(g)) as u8;
    let b = (256.0 * INTENSITY.clamp(b)) as u8;
    (r, g, b)
}
