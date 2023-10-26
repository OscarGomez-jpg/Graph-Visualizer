use macroquad::prelude::{Rect, Vec2};

#[derive(Debug)]
pub struct NodeGraph {
    pub val: usize,
    pub pos: Vec2,
    pub hitbox: Rect,
    pub adj: Vec<usize>,
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self {
            val: Default::default(),
            pos: Vec2::new(0.0, 0.0),
            hitbox: Rect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            adj: Default::default(),
        }
    }
}

impl NodeGraph {
    pub fn new(val: usize, pos_x: f32, pos_y: f32, width: f32, height: f32) -> Self {
        Self {
            val,
            pos: Vec2 { x: pos_x, y: pos_y },
            hitbox: Rect {
                x: pos_x,
                y: pos_y,
                w: width,
                h: height,
            },
            adj: Vec::new(),
        }
    }

    pub fn _is_colliding(&self, other: &Rect) -> bool {
        self.hitbox.overlaps(other)
    }

    pub fn _get_val(&self) -> &usize {
        &self.val
    }

    pub fn set_pos(&mut self, n_pos: Vec2) {
        self.hitbox.x = n_pos.x;
        self.hitbox.y = n_pos.y;
        self.pos.x = n_pos.x;
        self.pos.y = n_pos.y;
    }

    pub fn get_pos(&self) -> &Vec2 {
        &self.pos
    }

    pub fn get_adjs(&mut self) -> &mut Vec<usize> {
        &mut self.adj
    }

    pub fn get_hitbox(&self) -> &Rect {
        &self.hitbox
    }
}
