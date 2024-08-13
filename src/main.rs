mod command;
mod polls;
mod prompt;

use command::Command;

fn main() {
    println!("Welcome to Polli");
    let mut db = polls::PollDB::new();
    // infinite loop
    loop {
        let input = prompt::get_with_prompt("polli> ");
        // parse command
        let command = match Command::from_str(&input) {
            Some(cmd) => cmd,
            None => {
                println!("Invalid command");
                continue;
            }
        };
        command.execute(&mut db);
    }
}
