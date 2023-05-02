use std::sync::Arc;
use clap::{Parser, Subcommand};
use crate::{output::Output, session::Session};
pub use user::*;

pub mod user;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Input {
    #[clap(subcommand)]
    pub command: Command,
}

impl Input {
    pub async fn run(self, session: Arc<Session>) -> Output {
        self.command.run(session).await
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth { name: String },
    Login { name: String },
    Switch { to: String },

    User {
        name: String,
        #[clap(subcommand)]
        command: User
    },
    Project {
        id: u64,
        #[clap(subcommand)]
        command: ProjectCommand
    },
    Studio {
        id: u64,
        #[clap(subcommand)]
        command: StudioCommand
    },
    Forum {
        id: u64,
        #[clap(subcommand)]
        command: ForumCommand
    },
    ForumPost {
        id: u64,
        #[clap(subcommand)]
        command: ForumPostCommand
    },

    // aliases
    Follow { name: String },
    Unfollow { name: String },
    Love { id: u64 },
    Fav { id: u64 },
    Unlove { id: u64 },
    Unfav { id: u64 },
}

impl Command {
    pub async fn run(self, session: Arc<Session>) -> Output {
        match self {
            Self::Auth { name } => {
                session.add_auth(name).await.map(|_| Output::from("Adding authentication")).into()
            },

            Self::Switch { to } => {
                session.switch(to).await.map(|_| Output::from("Switching auth")).into()
            },

            Self::Login { name } => {
                session.login(name).await.map(|_| Output::from("Logging in")).into()
            },

            Self::User { name, command } => {
                command.run(name, session).await
            },

            Self::Follow { name } => {
                User::Follow.run(name, session).await
            }

            _ => todo!()
        }
    }
}


#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Love,
    Fav,
    Unlove,
    Unfav
}

#[derive(Debug, Subcommand)]
pub enum StudioCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Follow,
    Unfollow
}

#[derive(Debug, Subcommand)]
pub enum ForumCommand {
}


#[derive(Debug, Subcommand)]
pub enum ForumPostCommand {
}
