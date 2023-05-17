use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum User {
    /// General metadata
    Meta,
    /// Send comment
    Cmt {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    /// Follow
    Fol,
    /// Unfollow
    Unf,
    /// Check username (Valid / Invalid / Taken / Available)
    Check,
}

impl User {
    pub async fn run(self, name: String, session: Arc<Session>) -> Output {
        match self {
            Self::Meta => session.user_meta(&name).await.into(),
            Self::Fol => session.follow_user(&name).await.into(),
            Self::Unf => session.unfollow_user(&name).await.into(),
            Self::Cmt { content, parent, to } => {
                session.send_user_comment(&name, content, parent, to).await.into()
            },
            Self::Check => session.check_user(&name).await.into()
        }
    }
}
