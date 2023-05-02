use std::sync::Arc;
use clap::Subcommand;
use crate::{output::Output, session::Session};

#[derive(Debug, Subcommand)]
pub enum Project {
    Comment {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    Love,
    Fav,
    Unlove,
    Unfav
}

impl Project {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Love => session.love_project(id).await.into(),
            Self::Unlove => session.unlove_project(id).await.into(),
            Self::Fav => session.favorite_project(id).await.into(),
            Self::Unfav => session.unfavorite_project(id).await.into(),
            Self::Comment { content, parent, to } => {
                session.send_project_comment(id, &content, parent, to).await.into()
            },
        }
    }
}
