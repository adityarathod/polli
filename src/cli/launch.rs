use std::borrow::Borrow;

use diesel::SqliteConnection;
use diesel::BelongingToDsl;
use diesel::prelude::*;
use uuid::NoContext;
use uuid::Timestamp;
use uuid::Uuid;

use super::{Args, Subcommand};

use crate::models::{Poll, NewPoll, Choice, NewChoice};

pub fn run(args: &Args, conn: &mut SqliteConnection) {
    use crate::schema::{polls, choices};

    match args.command.borrow() {
        Subcommand::Create { question, options } => {
            // wrap in a transaction
            let result = conn.transaction(|tx| {
                // poll content
                let new_poll = NewPoll { 
                    external_id: Uuid::new_v7(Timestamp::now(NoContext)).to_string(),
                    question: (*question).clone(),
                };
                let poll = diesel::insert_into(polls::table)
                    .values(&new_poll)
                    .returning(Poll::as_returning())
                    .get_result(tx)
                    .expect("Error saving new poll");

                // insert individual options
                for opt in options {
                    let choice = NewChoice { poll_id: poll.id, choice_text: opt.to_string() };
                    diesel::insert_into(choices::table)
                        .values(&choice)
                        .execute(tx)?;
                }

                QueryResult::Ok(poll.external_id)
            });

            match result {
                Ok(id) => println!("Inserted poll with ID {} succcessfully", id),
                Err(err) => println!("Error: {:?}", err)
            }
        },
        Subcommand::Describe { id } => {
            use crate::schema::polls::dsl::{polls, external_id}; 

            let poll: Poll = polls
                .filter(external_id.eq(id))
                .select(Poll::as_select())
                .get_result(conn)
                .expect("No poll found with such ID");

            let poll_choices: Vec<Choice> = Choice::belonging_to(&poll)
                .select(Choice::as_select())
                .load(conn)
                .expect("No choices for this poll found");

            println!("Poll: {:?}", poll);
            println!("Choices: {:?}", poll_choices);
        }
        _ => todo!("Not implemented yet"),
    }
}
