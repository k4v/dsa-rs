use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

// Graph implemented as a vector of nodes
// where each node contains the reference
// of its adjacent nodes
pub type PtrGraph<T> = Vec<Rc<RefCell<PtrGraphNode<T>>>>;

pub trait PtrGraphValue: Clone + PartialEq + Default {}
impl <T: Clone + PartialEq + Default> PtrGraphValue for T {}

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
impl<T> PartialEq for PtrGraphNode<T> where T : PtrGraphValue {
    fn eq(&self, other: &Self)-> bool {
        self.value==other.value
    }
}

pub(crate) fn create_ptr_graph<T>() -> PtrGraph<T> where T : PtrGraphValue {
    let mut graph: PtrGraph<T> = Vec::new();

    println!("Creating graph with 10 nodes");
    for i in 0..10 {
        graph.push(PtrGraphNode::<T>::new(i, T::default()));
    }

    let _edge_lists: Vec::<Vec::<usize>> = vec![
        vec![1, 2], vec![0, 3, 4], vec![0, 5], vec![1, 5], vec![1],
        vec![2, 3, 6], vec![5, 9], vec![5, 8], vec![7], vec![6, 3]
    ];

    for (idx, edge_list) in _edge_lists.iter().enumerate() {
        let mut node = graph[idx].borrow_mut();
        
        println!("{} -> {:?}", idx, edge_list);

        // for..in iterator returns reference to element
        for edge_idx in edge_list {
            node.edges.push(Rc::clone(&graph[*edge_idx]));
        } 
    }

    graph
}
