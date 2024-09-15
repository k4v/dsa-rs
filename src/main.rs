pub mod types;
pub mod algos;
pub mod common;

use algos::dfs;
use common::{cli, runner::Runner};

fn main() {

    let args: cli::Args = argh::from_env();

    if args.runner().is_none() || args.runner().as_ref().unwrap().as_str() == "dfs" {
        println!();
        
        // Run DFS on graph implemented as node structs
        let dfs_runner = dfs::PtrDfsRunner::<u32>::init();
        dfs_runner.run();
    }
    
}
