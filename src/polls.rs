use std::collections::HashMap;

pub struct Poll {
    pub question: String,
    pub options: Vec<String>,
    pub votes: u32,
}

impl Poll {
    pub fn new(question: String, options: Vec<String>) -> Result<Poll, String> {
        if options.is_empty() {
            return Err("At least one option is required".into());
        }

        Ok(Poll {
            question,
            options,
            votes: 0,
        })
    }
}

type PollID = String;

pub struct PollDB {
    polls: HashMap<PollID, Poll>,
}

impl PollDB {
    pub fn new() -> PollDB {
        PollDB {
            polls: HashMap::new(),
        }
    }

    pub fn add_poll(&mut self, id: PollID, poll: Poll) {
        self.polls.insert(id, poll);
    }

    pub fn list_polls(&self) {
        for poll in self.polls.iter() {
            println!(
                "[{}] {} (options: {:?}) (votes: {:?})",
                poll.0, poll.1.question, poll.1.options, poll.1.votes
            );
        }
    }
}
