use std::sync::Arc;
use colored::Colorize;
use crate::{input::ProjectCommand, store::Store};

pub async fn entry(id: u64, command: ProjectCommand, store: Store, session: Arc<s2rs::Session>) {
    let this = session.project(id);
    match command {
        ProjectCommand::Comment { content, to, parent } => {
            this.send_comment(content.as_str()).await.unwrap();
            println!("{} left a comment on project #{}.",
                "Successfully".green(),
                id.to_string().yellow(),
            );
        },
        ProjectCommand::Fav => {
            this.love().await.unwrap();
            println!("{} loved {}.",
                "Successfully".green(),
                id.to_string().yellow()
            );
        },

        ProjectCommand::Love => {
            this.favorite().await.unwrap();
            println!("{} favorited {}.",
                "Successfully".green(),
                id.to_string().yellow()
            );
        },

        ProjectCommand::Unlove => {
            this.unlove().await.unwrap();
            println!("Success");
        },

        ProjectCommand::Unfav => {
            this.unfavorite().await.unwrap();
            println!("Success");
        },
    }
}