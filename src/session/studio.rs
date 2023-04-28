use std::sync::Arc;
use colored::Colorize;
use crate::{input::StudioCommand, store::Store};

pub async fn entry(id: u64, command: StudioCommand, store: Store, session: Arc<s2rs::Session>) {
    let this = session.studio(id);
    match command {
        StudioCommand::Comment { content, to, parent } => {
            this.send_comment(&content).await.unwrap();
            println!("{} left a comment on studio #{}.",
                "Successfully".green(),
                id.to_string().yellow(),
            );
        },

        StudioCommand::Follow => {
            this.follow().await.unwrap();
        },

        StudioCommand::Unfollow => {
            this.unfollow().await.unwrap();
        }
    }
}