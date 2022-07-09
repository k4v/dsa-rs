use std::collections::HashSet;

use crate::types::graph::ALGraph;
use crate::types::graph::ALGraphValue;

fn create_graph() -> ALGraph<u32> {

    let mut graph = ALGraph::<u32>::new();

    graph.insert(0, HashSet::from([1, 2]));
    graph.insert(1, HashSet::from([0, 3, 4]));
    graph.insert(2, HashSet::from([0, 5]));
    graph.insert(3, HashSet::from([1, 5]));
    graph.insert(4, HashSet::from([1]));
    graph.insert(5, HashSet::from([2, 3, 6]));
    graph.insert(6, HashSet::from([5, 9]));
    graph.insert(7, HashSet::from([5, 8]));
    graph.insert(8, HashSet::from([7]));
    graph.insert(9, HashSet::from([6]));

    println!("Creating graph with {} nodes", graph.keys().len());
    for (_node, _edges) in &graph {
        println!("{} -> {:?}", _node, _edges);
    }

    graph
}

fn run_dfs<T: ALGraphValue>(graph: &ALGraph<T>) {
    let mut visited = HashSet::<T>::new();

    for (vertex, _) in graph.into_iter() {
        if !visited.contains(vertex) {
            run_dfs_internal(graph, vertex, 0, &mut visited);
        }
    }
}

fn run_dfs_internal<T: ALGraphValue>(
    graph: &ALGraph<T>, root: &T, level: u32, visited: &mut HashSet<T>) {

    println!("{}{}", (0..level).map(|_| ".").collect::<String>(), root);

    visited.insert(*root);

    let children = graph.get(root);
    for child in children.unwrap_or(&mut HashSet::<T>::new()) {
        if !visited.contains(child) {
            visited.insert(*child);
            run_dfs_internal(graph, child, level + 1, visited);
        }
    }
}

pub fn test_dfs() {
    let graph = create_graph();
    println!();
    run_dfs(&graph);
}
