use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum Post {
    Cnt,
    Edit { content: String },
}

impl Post {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Edit { content } => session.edit_post(id, &content).await.into(),
            Self::Cnt => session.post_content(id).await.into(),
        }
    } 
}
