#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn iter() -> [Axis; 3] {
        [Axis::X, Axis::Y, Axis::Z]
    }
}
