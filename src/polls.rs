use std::collections::HashMap;

pub struct Poll {
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
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
