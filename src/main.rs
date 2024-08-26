mod models;
mod schema;
mod cli;

use std::borrow::Borrow;

use clap::Parser;
use diesel::prelude::*;
use diesel::{Connection, ExpressionMethods, SqliteConnection};

use self::models::*;
use self::schema::polls::dsl::*;

fn main() {
    let args = cli::Args::parse();
    let mut conn = SqliteConnection::establish(args.db_file.borrow())
        .unwrap_or_else(|_| panic!("Error connecting to {}", args.db_file));
    println!("Welcome to Polli");
    let results = polls
        .filter(question.ne(""))
        .select(Poll::as_select())
        .load(&mut conn)
        .expect("Error loading polls");
    println!("Results: {:?}", results);
    cli::run(&args);
}
