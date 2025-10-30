mod cli;
mod db;

use clap::Parser; // This import is now valid and brings the trait into scope.
use crate::cli::db_cli::DBCli;

fn main() {
    let args = DBCli::parse(); // The `parse()` method is now available.
    args.run_cli();
}
