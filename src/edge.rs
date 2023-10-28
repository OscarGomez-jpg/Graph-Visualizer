use macroquad::prelude::{Color, WHITE};

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: usize,
    pub color: Color,
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: usize) -> Self {
        Self {
            from,
            to,
            weight,
            color: WHITE,
        }
    }
}
