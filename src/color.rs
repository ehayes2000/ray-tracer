use crate::vec3::Color;
use std::io::Write;

use super::interval::Interval;

static INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

fn linear_to_gamma(component: f64) -> f64 {
    component
    // if component > 0.0 {
    //     f64::sqrt(component)
    // } else {
    //     component
    // }
}

pub fn write_color<F>(f: &mut F, color: &Color) -> Result<(), std::io::Error>
where
    F: Write,
{
    let r = linear_to_gamma(color.0);
    let g = linear_to_gamma(color.1);
    let b = linear_to_gamma(color.2);
    let r = (256.0 * INTENSITY.clamp(r)) as i32;
    let g = (256.0 * INTENSITY.clamp(g)) as i32;
    let b = (256.0 * INTENSITY.clamp(b)) as i32;

    writeln!(f, "{} {} {}", r, g, b)
}
