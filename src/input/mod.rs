use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Input {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth { name: String },
    Login { name: String },
    Switch { to: String },

    User {
        name: String,
        #[clap(subcommand)]
        command: UserCommand
    },

    Follow { name: String },
    Unfollow { name: String },

    Love { id: u64 },
    Fav { id: u64 },

}

#[derive(Debug, Subcommand)]
pub enum UserCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    }
}