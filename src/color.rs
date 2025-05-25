use crate::vec3::Color;
use std::io::Write;

pub fn write_color<F>(f: &mut F, color: &Color) -> Result<(), std::io::Error>
where
    F: Write,
{
    let r = (255.999_f64 * color.x()) as i32;
    let g = (255.999_f64 * color.y()) as i32;
    let b = (255.999_f64 * color.z()) as i32;

    writeln!(f, "{} {} {}", r, g, b)
}
