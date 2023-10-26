mod graph;
mod node_graph;

use macroquad::{
    prelude::{
        is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton, Vec2, BLACK,
        WHITE,
    },
    shapes::draw_circle,
    window::{clear_background, next_frame},
};

use graph::Graph;

#[macroquad::main("Graphs")]
async fn main() {
    let mut graph = Graph::new();
    let mut dragged_node = None;

    for i in 0..10 {
        graph.add_node(i, i as f32 * 20.0, i as f32 * 20.0, 20.0, 20.0);
    }

    loop {
        clear_background(BLACK);

        //Draw
        for (_key, val) in graph.nodes.borrow().iter() {
            draw_circle(val.get_pos().x, val.get_pos().y, val.get_hitbox().w, WHITE);
        }
        //End of draw

        //Move nodes around
        let pos = Vec2::from(mouse_position());

        if is_mouse_button_down(MouseButton::Left) {
            if dragged_node.is_none() {
                for (key, val) in graph.nodes.borrow_mut().iter_mut() {
                    if val.hitbox.contains(pos) {
                        dragged_node = Some(*key)
                    }
                }
            } else if let Some(key) = dragged_node {
                if let Some(node) = graph.nodes.borrow_mut().get_mut(&key) {
                    node.set_pos(pos);
                }
            }
        } else if is_mouse_button_released(MouseButton::Left) {
            dragged_node = None;
        }
        //end of move nodes around

        next_frame().await
    }
}
