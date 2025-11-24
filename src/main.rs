mod cli;
mod constant;
mod db;
mod file;

use crate::cli::db_cli::DBCli;
use clap::Parser;
// This import is now valid and brings the trait into scope.

fn main() {
    let args = DBCli::parse(); // The `parse()` method is now available.
    args.run_cli();
}
