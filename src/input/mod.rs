use std::sync::Arc;
use clap::{Parser, Subcommand};
use crate::{output::Output, session::Session};
pub use user::*;
pub use project::*;
pub use studio::*;

pub mod user;
pub mod project;
pub mod studio;

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
        command: Option<User>
    },
    Project {
        id: u64,
        #[clap(subcommand)]
        command: Project
    },
    Studio {
        id: u64,
        #[clap(subcommand)]
        command: Studio
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
    Check { name: String },

    Love { id: u64 },
    Fav { id: u64 },
    Unlove { id: u64 },
    Unfav { id: u64 },
}

impl Command {
    pub async fn run(self, session: Arc<Session>) -> Output {
        match self {
            Self::Auth { name } => session.add_auth(name).await.into(),
            Self::Switch { to } => session.switch(to).await.into(),
            Self::Login { name } => session.login(name).await.into(),

            Self::User { name, command } => match command {
                Some(command) => command.run(name, session).await,
                None => User::Meta.run(name, session).await
            },
            Self::Project { id, command } => command.run(id, session).await,
            Self::Studio { id, command } => command.run(id, session).await,

            Self::Follow { name } => User::Follow.run(name, session).await,
            Self::Unfollow { name } => User::Unfollow.run(name, session).await,
            Self::Check { name } => User::Check.run(name, session).await,

            Self::Love { id } => Project::Love.run(id, session).await,
            Self::Unlove { id } => Project::Unlove.run(id, session).await,
            Self::Fav { id } => Project::Fav.run(id, session).await,
            Self::Unfav { id } => Project::Unfav.run(id, session).await,

            _ => todo!()
        }
    }
}


#[derive(Debug, Subcommand)]
pub enum ForumCommand {
}


#[derive(Debug, Subcommand)]
pub enum ForumPostCommand {
}
