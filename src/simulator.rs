use macroquad::{
    prelude::{
        is_mouse_button_down, is_mouse_button_released, mouse_position, rand, MouseButton, Vec2,
        WHITE,
    },
    shapes::{draw_circle, draw_line},
};

use crate::{graph::Graph, node_graph::NodeGraph};

pub struct Simulator {
    graph: Graph,
    dragged_node: Option<usize>,
}

impl Simulator {
    pub fn default() -> Self {
        Self {
            graph: Graph::new(),
            dragged_node: None,
        }
    }

    pub fn initialize(&mut self) {
        for i in 0..10 {
            self.graph
                .add_node(i, i as f32 * 20.0, i as f32 * 20.0, 20.0, 20.0);
        }

        let mut added = 5;
        while added > 0 {
            let from = rand::gen_range(0, 10);
            let to = rand::gen_range(0, 10);

            if to != from {
                self.graph.add_simple_edge(from as usize, to as usize);
                added -= 1;
            }
        }
    }

    pub fn draw(&self) {
        let nodes = self.graph.nodes.borrow();
        let vals: Vec<&NodeGraph> = nodes.values().collect();

        for val in vals {
            draw_circle(val.get_pos().x, val.get_pos().y, val.get_hitbox().w, WHITE);

            for adj in &val.adj {
                if let Some(act) = nodes.get(adj) {
                    draw_line(
                        act.get_pos().x,
                        act.get_pos().y,
                        val.get_pos().x,
                        val.get_pos().y,
                        1.0,
                        WHITE,
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        let pos = Vec2::from(mouse_position());

        if is_mouse_button_down(MouseButton::Left) {
            if self.dragged_node.is_none() {
                for (key, val) in self.graph.nodes.borrow_mut().iter_mut() {
                    if val.hitbox.contains(pos) {
                        self.dragged_node = Some(*key)
                    }
                }
            } else if let Some(key) = self.dragged_node {
                if let Some(node) = self.graph.nodes.borrow_mut().get_mut(&key) {
                    node.set_pos(pos);
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            self.dragged_node = None;
        }
    }
}
