use std::sync::Arc;
use clap::{Parser, Subcommand};
use crate::{output::Output, session::Session};
pub use user::*;
pub use project::*;
pub use studio::*;
pub use topic::*;
pub use post::*;

pub mod user;
pub mod project;
pub mod studio;
pub mod topic;
pub mod post;

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
    /// Add authentication through cookies
    Auth { name: String },
    /// Add authentication through username and password
    Login { name: String },
    /// Switch current authentication to another
    Switch { to: String },
    /// Reset (delete) all of the ClI state, including sessions and encryption key
    Reset,
    /// Remove particular authentication session
    Unauth { name: String },
    User {
        name: String,
        #[clap(subcommand)]
        command: Option<User>
    },
    Project {
        id: u64,
        #[clap(subcommand)]
        command: Option<Project>
    },
    Studio {
        id: u64,
        #[clap(subcommand)]
        command: Option<Studio>
    },
    /// Forum topic
    Topic {
        id: u64,
        #[clap(subcommand)]
        command: Topic
    },
    /// Forum post
    Post {
        id: u64,
        #[clap(subcommand)]
        command: Option<Post>
    },

    // aliases
    /// Follow user (alias)
    Follow { name: String },
    /// Unfollow user (alias)
    Unfollow { name: String },
    /// Check username (Valid / Invalid / Taken / Available) (alias)
    Check { name: String },
    /// Love project (alias)
    Love { id: u64 },
    /// Favorite project (alias)
    Fav { id: u64 },
    /// Unlove project (alias)
    Unlove { id: u64 },
    /// Unfavorite project (alias)
    Unfav { id: u64 },
}

impl Command {
    pub async fn run(self, session: Arc<Session>) -> Output {
        match self {
            Self::Reset => session.reset().await,
            Self::Unauth { name } => session.remove_auth(&name).await.into(),
            Self::Auth { name } => session.add_auth(name).await.into(),
            Self::Switch { to } => session.switch(to).await.into(),
            Self::Login { name } => session.login(name).await.into(),

            Self::User { name, command } => match command {
                Some(command) => command.run(name, session).await,
                None => User::Meta.run(name, session).await
            },
            Self::Project { id, command } => match command {
                Some(command) => command.run(id, session).await,
                None => Project::Meta.run(id, session).await,
            },

            Self::Studio { id, command } => match command {
                Some(command) => command.run(id, session).await,
                None => Studio::Meta.run(id, session).await,
            },

            Self::Follow { name } => User::Fol.run(name, session).await,
            Self::Unfollow { name } => User::Unf.run(name, session).await,
            Self::Check { name } => User::Check.run(name, session).await,

            Self::Love { id } => Project::Lov.run(id, session).await,
            Self::Unlove { id } => Project::Unl.run(id, session).await,
            Self::Fav { id } => Project::Fav.run(id, session).await,
            Self::Unfav { id } => Project::Unf.run(id, session).await,

            Self::Post { id, command } => match command {
                Some(command) => command.run(id, session).await,
                None => Post::Cnt.run(id, session).await
            },

            Self::Topic { id, command } => command.run(id, session).await,
        }
    }
}
