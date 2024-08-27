mod models;
mod schema;
mod cli;

use std::borrow::Borrow;

use clap::Parser;
use diesel::{Connection, SqliteConnection};

fn main() {
    let args = cli::Args::parse();
    let mut conn = SqliteConnection::establish(args.db_file.borrow())
        .unwrap_or_else(|_| panic!("Error connecting to {}", args.db_file.clone()));
    println!("Welcome to Polli. Connected to DB file: {}", args.db_file);
    cli::run(&args, &mut conn);
}
