use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "dev", about = "polli means poll interface. an interface to conduct polls.", long_about = None)]
pub struct Args {
    #[arg(short = 'f', long, help = "The path to the database file")]
    pub db_file: String,
    #[command(subcommand)]
    pub command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum Subcommand {
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
            help = "The ID of the poll to describe"
        )]
        id: String,
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

