use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{edge::Edge, node_graph::NodeGraph};

pub struct Graph {
    pub nodes: Rc<RefCell<HashMap<usize, NodeGraph>>>,
    pub idx: usize,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Rc::new(RefCell::new(HashMap::new())),
            idx: 0,
        }
    }

    pub fn add_node(&mut self, pos_x: f32, pos_y: f32, width: f32, height: f32) {
        let new = NodeGraph::new(self.idx, pos_x, pos_y, width, height);
        let act = self.idx;
        self.nodes.borrow_mut().insert(act, new);
        self.idx += 1;
    }

    pub fn add_simple_edge(&mut self, val1: usize, val2: usize) {
        self.add_directed_edge(val1, val2);
        self.add_directed_edge(val2, val1);
    }

    pub fn add_directed_edge(&mut self, val1: usize, val2: usize) {
        let connecting = Rc::clone(&self.nodes);
        let mut connected_nodes = connecting.borrow_mut();
        let node1 = connected_nodes
            .entry(val1)
            .or_insert_with(|| NodeGraph::new(val1, 0.0, 0.0, 0.0, 0.0));
        let n_edge = Edge::new(val1, val2, 0);
        node1.adj.push(n_edge);
    }

    pub fn bfs(&mut self, start: usize) {}
}
