use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

trait DfsTraverse {
    fn dfs<T>() -> String;
}

// Simple graph implementation
// as an adjacency list from a
// parent node to set of nodes
pub type ALGraph<T> = HashMap<T, HashSet<T>>;

pub trait ALGraphValue: Copy + Eq + Hash + Display {}
impl <T: Copy + Eq + Hash + Display> ALGraphValue for T {}


// Graph implemented as a vector of nodes
// where each node contains the reference
// of its adjacent nodes
pub type PtrGraph<T> = Vec<Rc<RefCell<PtrGraphNode<T>>>>;

pub trait PtrGraphValue: Clone + PartialEq {}
impl <T: Clone + PartialEq> PtrGraphValue for T {}

#[derive(Debug, Eq)]
pub struct PtrGraphNode<T: PtrGraphValue> {
    pub idx: usize,
    pub value: T,
    pub edges: PtrGraph<T>
}

impl<T: PtrGraphValue> PtrGraphNode<T> {
    pub fn new(_idx: usize, _value: T) -> Rc<RefCell<PtrGraphNode<T>>> {
        Rc::new(RefCell::new(PtrGraphNode {
            idx: _idx,
            value: _value.clone(),
            edges: Vec::new()
        }))
    }
}

// Any way to not have to write the entire type qualification each time?
impl<T: PtrGraphValue> PartialEq for PtrGraphNode<T> {
    fn eq(&self, other: &Self)-> bool {
        self.value==other.value
    }
}
