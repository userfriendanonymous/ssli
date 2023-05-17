use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum Topic {
    Post { content: String }
}

impl Topic {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Post { content } => session.send_topic_post(id, &content).await.into(),
        }
    } 
}