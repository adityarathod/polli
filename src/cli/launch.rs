use std::borrow::Borrow;

use super::{Args, Subcommand};

pub fn run(args: &Args) {
    match args.command.borrow() {
        Subcommand::Create { question, options } => {
            println!("{:?}", question);


        },
        _ => todo!("Not implemented yet"),
    }
}
