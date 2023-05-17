use std::sync::Arc;
use clap::Subcommand;
use crate::{output::Output, session::Session};

#[derive(Debug, Subcommand)]
pub enum Project {
    /// Send comment
    Cmt {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    /// General metadata
    Meta,
    /// Love
    Lov,
    /// Favorite
    Fav,
    /// Unlove
    Unl,
    /// Unfavorite
    Unf
}

impl Project {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Meta => session.project_meta(id).await.into(),
            Self::Lov => session.love_project(id).await.into(),
            Self::Unl => session.unlove_project(id).await.into(),
            Self::Fav => session.favorite_project(id).await.into(),
            Self::Unf => session.unfavorite_project(id).await.into(),
            Self::Cmt { content, parent, to } => {
                session.send_project_comment(id, content, parent, to).await.into()
            },
        }
    }
}
