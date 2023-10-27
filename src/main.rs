mod graph;
mod node_graph;
mod simulator;

use macroquad::{
    prelude::BLACK,
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

        next_frame().await
    }
}
