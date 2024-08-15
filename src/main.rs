mod polls;

use clap::{Parser, Subcommand};
use polls::{Poll, PollDB};

#[derive(Parser, Debug)]
#[command(version = "dev", about = "polli means poll interface. an interface to conduct polls.", long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: CliSubcommand,
}

#[derive(Subcommand, Debug)]
enum CliSubcommand {
    Create {
        #[arg(short, long,
            value_parser = clap::builder::NonEmptyStringValueParser::new(),
            help = "The question to ask in the poll"
        )]
        question: String,
        #[arg(short, long,
            num_args = 1..,
            required = true,
            help = "The options to provide for the poll; at least one option is required"
        )]
        options: Vec<String>,
    },
    Describe {
        #[arg(short, long,
            value_parser = clap::builder::NonEmptyStringValueParser::new(),
            help = "The ID of the poll to describe (optional)"
        )]
        id: Option<String>,
    },
    Update {
        #[arg(short, long,
            value_parser = clap::builder::NonEmptyStringValueParser::new(),
            help = "The question to ask in the poll"
        )]
        question: Option<String>,
        #[arg(short, long,
            num_args = 1..,
            help = "The options to provide for the poll; at least one option is required"
        )]
        options: Option<Vec<String>>,
    },
    Delete {
        #[arg(short, long,
            value_parser = clap::builder::NonEmptyStringValueParser::new(),
            help = "The ID of the poll to delete"
        )]
        id: String,
    },
    Vote {
        #[arg(short, long,
            value_parser = clap::builder::NonEmptyStringValueParser::new(),
            help = "The ID of the poll to vote on"
        )]
        id: String,
        #[arg(short, long, 
            value_parser = clap::builder::NonEmptyStringValueParser::new(), 
            help = "The option to vote for"
        )]
        option: String,
    },
}

fn main() {
    let args = CliArgs::parse();
    let mut db = PollDB::new();
    println!("Welcome to Polli");
    // let mut db = polls::PollDB::new();
    match args.command {
        CliSubcommand::Create { question, options } => {
            let id = format!("{:x}", rand::random::<u32>());
            let poll = Poll::new(question, options);
            db.add_poll(id, poll);
            db.list_polls();
        }
        _ => println!("Command not implemented"),
    }
}
