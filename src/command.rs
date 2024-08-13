use crate::{
    polls::{Poll, PollDB},
    prompt::get_with_prompt,
};

pub enum Command {
    Create,
    Describe,
    Update,
    Delete,
    Vote,
    Help,
}

impl Command {
    pub fn from_str(input: &str) -> Option<Command> {
        match input.to_lowercase().as_str() {
            "c" | "create" => Some(Command::Create),
            "dc" | "describe" => Some(Command::Describe),
            "u" | "update" => Some(Command::Update),
            "d" | "delete" => Some(Command::Delete),
            "v" | "vote" => Some(Command::Vote),
            "h" | "help" | "?" => Some(Command::Help),
            _ => None,
        }
    }

    pub fn execute(self, db: &mut PollDB) {
        match self {
            Command::Create => {
                let id = format!("{:x}", rand::random::<u32>());
                let question = get_with_prompt("Enter your question: ");
                let mut options: Vec<String> = Vec::new();
                loop {
                    let option = get_with_prompt("Enter an option (or leave blank to stop): ");
                    if option.is_empty() {
                        break;
                    }
                    options.push(option);
                }
                let poll = Poll {
                    question,
                    options,
                    votes: vec![0; 5],
                };
                db.add_poll(id, poll);
                db.list_polls();
            }
            Command::Describe => println!("Describe"),
            Command::Update => println!("Update"),
            Command::Delete => println!("Delete"),
            Command::Vote => println!("Vote"),
            Command::Help => println!("Help"),
        }
    }
}
