use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum User {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Follow,
    Unfollow,
}

impl User {
    pub async fn run(self, name: String, session: Arc<Session>) -> Output {
        match self {
            Self::Comment { content, to, parent } => {
                session.comment_user(name, content, to, parent).await.into()
            },

            Self::Follow => {
                session.follow_user(name).await.map(|_| Output::from("Following")).into()
            },

            Self::Unfollow => {
                session.unfollow_user(name).await.map(|_| Output::from("Unfollow")).into()
            }
        }
    }
}