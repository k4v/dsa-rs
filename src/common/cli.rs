#![allow(dead_code)]

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(description = "List of arguments to run through the various available data structures and algorithms available in this set")]
pub(crate) struct Args {

    #[argh(option, short = 'r', description = "which algorithm or data structure to run")]
    runner: Option<String>
}

impl Args {
    pub(crate) fn runner(&self) -> &Option<String> {
        &self.runner
    }
}