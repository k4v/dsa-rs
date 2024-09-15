use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::runner::Runner;
use crate::types::ptr_graph::create_ptr_graph;
use crate::types::ptr_graph::PtrGraphNode;
use crate::types::ptr_graph::PtrGraph;
use crate::types::ptr_graph::PtrGraphValue;


pub(crate) struct PtrDfsRunner<T: PtrGraphValue>  {
    graph: PtrGraph<T>
}

impl<T> PtrDfsRunner<T> where T : PtrGraphValue {

    fn run_dfs(&self) {
        let mut visited = HashSet::<usize>::new();
    
        for node in &self.graph {
            // borrow() checks at runtime whether the value is safe to borrow.
            // Task panic and exits if not.
            if !visited.contains(&node.borrow().idx) {
                Self::run_dfs_internal(&node, 0, &mut visited);
            }
        }
    }
    
    fn run_dfs_internal(
        root: &Rc<RefCell<PtrGraphNode<T>>>,
        level: u32, visited: &mut HashSet<usize>) {
    
        // Print spaces equal to level, then print node
        println!("{}{}", (0..level).map(|_| ".").collect::<String>(), root.borrow().idx);
    
        // borrow() returns the RefCell,
        // .idx is able to do an implicit borrow and return the value from PtrGraphNode inside the RefCell
        // So only 1 borrow required.
        visited.insert(root.borrow().idx);
    
        let children = &root.borrow().edges;
        for child in children {
    
            if !visited.contains(&child.borrow().idx) {
                Self::run_dfs_internal(child, level + 1, visited);
            }
        }
    }    
}

impl<T> Runner for PtrDfsRunner<T> where T : PtrGraphValue {
    fn init() -> PtrDfsRunner<T> {
        Self {
            graph: create_ptr_graph(),
        }
    }
    
    fn run(&self) {
        self.run_dfs();
    }
    
}
