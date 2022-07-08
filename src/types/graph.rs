use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;


pub trait GraphValue: Copy + Eq + Hash + Display {}
impl <T: Copy + Eq + Hash + Display> GraphValue for T {}

// Simple graph implementation
// as an adjacency list from a
// parent node to set of nodes
pub type ALGraph<T> = HashMap<T, HashSet<T>>;

// Graph implemented as a vector of nodes
// where each node contains the reference
// of its adjacent nodes
pub type PtrGraph<T> = Vec<Rc<RefCell<PtrGraphNode<T>>>>;

#[derive(Debug, Eq)]
pub struct PtrGraphNode<T: Clone + PartialEq> {
    pub idx: usize,
    pub value: T,
    pub edges: PtrGraph<T>
}

impl<T: Clone + PartialEq> PtrGraphNode<T> {
    pub fn new(_idx: usize, _value: T) -> Rc<RefCell<PtrGraphNode<T>>> {
        Rc::new(RefCell::new(PtrGraphNode {
            idx: _idx,
            value: _value.clone(),
            edges: Vec::new()
        }))
    }
}

// Any way to not have to write the entire type qualification each time?
impl<T: Clone + PartialEq> PartialEq for PtrGraphNode<T> {
    fn eq(&self, other: &Self)-> bool {
        self.value==other.value
    }
}
