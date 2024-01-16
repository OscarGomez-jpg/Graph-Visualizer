use macroquad::{
    prelude::{
        is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position,
        MouseButton, Vec2, GREEN, RED, WHITE,
    },
    shapes::{draw_circle, draw_line},
    window::{screen_height, screen_width},
};

use crate::{graph::Graph, node_graph::NodeGraph};

pub struct Simulator {
    graph: Graph,
    selected_nodes: Option<Vec<usize>>,
    dragged_node: Option<usize>,
}

impl Simulator {
    pub fn default() -> Self {
        Self {
            graph: Graph::new(),
            selected_nodes: None,
            dragged_node: None,
        }
    }

    pub fn initialize(&mut self) {
        // let height = 40.0;
        // let width = 40.0;
        // let size = 30.0;
        // for i in 1..=10 {
        //     for j in 1..=10 {
        //         self.graph
        //             .add_node(i, j as f32 * width, i as f32 * height, size, size);
        //     }
        // }

        // let mut added = 10;
        // while added > 0 {
        //     let from = rand::gen_range(0, 10);
        //     let to = rand::gen_range(0, 10);

        //     if to != from {
        //         self.graph.add_simple_edge(from as usize, to as usize);
        //         added -= 1;
        //     }
        // }
    }

    pub fn draw(&self) {
        let nodes = self.graph.nodes.borrow();
        let vals: Vec<&NodeGraph> = nodes.values().collect();

        for val in vals {
            draw_circle(
                val.get_pos().x + val.get_hitbox().w / 2.0,
                val.get_pos().y + val.get_hitbox().w / 2.0,
                val.get_hitbox().w / 2.0,
                val.color,
            );

            draw_circle(val.get_hitbox().x, val.get_hitbox().y, 5.0, RED);

            for adj in &val.adj {
                if let Some(act) = nodes.get(&adj.to) {
                    draw_line(
                        act.get_pos().x,
                        act.get_pos().y,
                        val.get_pos().x,
                        val.get_pos().y,
                        1.0,
                        act.color,
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        let mouse_pos = Vec2::from(mouse_position());
        self.dragg_node(mouse_pos);
        self.select_nodes(mouse_pos);
    }

    pub fn add_node(&mut self) {
        self.graph
            .add_node(screen_width() / 2.0, screen_height() / 2.0, 40.0, 40.0)
    }

    pub fn add_edge(&mut self) {
        if let Some(selected) = &self.selected_nodes {
            if selected.len() == 0 {
                return;
            }
            for i in 0..(selected.len() - 1) {
                self.graph.add_simple_edge(selected[i], selected[i + 1]);
            }
        }
    }

    fn select_nodes(&mut self, mouse_pos: Vec2) {
        if is_mouse_button_pressed(MouseButton::Left) {
            if self.selected_nodes.is_none() {
                self.selected_nodes = Some(Vec::new());
            }
            for (key, node) in self.graph.nodes.borrow_mut().iter_mut() {
                if node.hitbox.contains(mouse_pos) {
                    if let Some(sel) = &mut self.selected_nodes {
                        sel.push(*key);
                    }

                    node.color = GREEN;
                }
            }
        } else if is_mouse_button_pressed(MouseButton::Right) {
            if self.selected_nodes.is_some() {
                let indexes = self.selected_nodes.clone().unwrap();
                for i in indexes {
                    let mut node = self.graph.nodes.borrow_mut();
                    node.get_mut(&i).unwrap().color = WHITE;
                }
            }

            self.selected_nodes = None;
        }
    }

    fn dragg_node(&mut self, mouse_pos: Vec2) {
        if is_mouse_button_down(MouseButton::Left) {
            if self.dragged_node.is_none() {
                for (key, val) in self.graph.nodes.borrow_mut().iter_mut() {
                    if val.hitbox.contains(mouse_pos) {
                        self.dragged_node = Some(*key)
                    }
                }
            } else if let Some(key) = self.dragged_node {
                if let Some(node) = self.graph.nodes.borrow_mut().get_mut(&key) {
                    node.update_position_and_hitbox(mouse_pos);
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            self.dragged_node = None;
        }
    }
}
