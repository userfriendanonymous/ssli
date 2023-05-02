use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum Studio {
    Comment {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    Follow,
    Unfollow
}

impl Studio {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Follow => session.follow_studio(id).await.into(),
            Self::Unfollow => session.unfollow_studio(id).await.into(),
            Self::Comment { content, parent, to } => session.send_studio_comment(id, &content, parent, to).await.into()
        }
    }
}