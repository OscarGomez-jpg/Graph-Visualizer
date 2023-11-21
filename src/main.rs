mod edge;
mod graph;
mod node_graph;
mod simulator;

use macroquad::{
    prelude::BLACK,
    ui::root_ui,
    window::{clear_background, next_frame},
};

use simulator::Simulator;

#[macroquad::main("Graphs")]
async fn main() {
    let mut main_graph = Simulator::default();
    main_graph.initialize();

    loop {
        clear_background(BLACK);

        main_graph.draw();
        main_graph.update();

        if root_ui().button(None, "Add node") {
            main_graph.add_node();
        } else if root_ui().button(None, "Add edge") {
            main_graph.add_edge();
        }

        next_frame().await
    }
}
