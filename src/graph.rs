use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::node_graph::NodeGraph;

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

    pub fn add_node(&mut self, val: usize, pos_x: f32, pos_y: f32, width: f32, height: f32) {
        let new = NodeGraph::new(val, pos_x, pos_y, width, height);
        let act = self.idx;
        self.nodes.borrow_mut().insert(act, new);
        self.idx += 1;
    }

    pub fn add_edge(&mut self, val1: usize, val2: usize) {
        self.nodes
            .borrow_mut()
            .get_mut(&val1)
            .unwrap()
            .get_adjs()
            .push(val2);

        self.nodes
            .borrow_mut()
            .get_mut(&val2)
            .unwrap()
            .get_adjs()
            .push(val1);
    }
}
