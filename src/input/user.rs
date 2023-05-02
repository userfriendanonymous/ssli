use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum User {
    Meta,
    Comment {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    Follow,
    Unfollow,
    Check,
}

impl User {
    pub async fn run(self, name: String, session: Arc<Session>) -> Output {
        match self {
            Self::Meta => session.user_meta(&name).await.into(),
            Self::Follow => session.follow_user(&name).await.into(),
            Self::Unfollow => session.unfollow_user(&name).await.into(),
            Self::Comment { content, parent, to } => {
                session.send_user_comment(&name, content, parent, to).await.into()
            },
            Self::Check => session.check_user(&name).await.into()
        }
    }
}
