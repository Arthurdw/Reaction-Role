mod commands;
mod formatting;
mod io;
mod levels;
mod message_handler;

use anyhow::Error;
use commands::{level, top};
pub use io::Database as ExpSystemDatabase;
pub use message_handler::MessageHandler as ExpSystemMessageHandler;
use poise::Command;

use crate::run::Data;

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![level(), top()]
}
