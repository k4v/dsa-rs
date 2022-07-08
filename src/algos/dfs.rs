use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;

use crate::types::graph::PtrGraphNode;
use crate::types::graph::PtrGraph;

fn create_graph() -> PtrGraph<u32> {
    let mut graph: PtrGraph<u32> = Vec::new();

    println!("Creating graph with 10 nodes");
    for i in 0..10 {
        graph.push(PtrGraphNode::<u32>::new(i, i.try_into().unwrap()));
    }

    let mut _edge_lists: Vec::<Vec::<usize>> = vec![
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

fn run_dfs(graph: PtrGraph<u32>) {
    let mut visited = HashSet::<usize>::new();

    for node in graph {
        // borrow() checks at runtime whether the value is safe to borrow.
        // Task panic and exits if not.
        if !visited.contains(&node.borrow().idx) {
            run_dfs_internal(&node, 0, &mut visited);
        }
    }
}

fn run_dfs_internal(
    root: &Rc<RefCell<PtrGraphNode<u32>>>,
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
            run_dfs_internal(child, level + 1, visited);
        }
    }
}

pub fn test_dfs() {
    let graph = create_graph();
    println!();
    run_dfs(graph);
}
