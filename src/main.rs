pub mod types;
pub mod algos;

fn main() {
    // Run DFS on graph created as value map
    algos::dfs_al::test_dfs();

    println!();

    // Run DFS on graph implemented as node structs
    algos::dfs::test_dfs();
}
