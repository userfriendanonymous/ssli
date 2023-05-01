use std::sync::Arc;
use colored::Colorize;
use crate::{input::UserCommand, store::Store, output::{Output, OutputErr}};

pub async fn entry(name: String, command: UserCommand, store: Store, session: Arc<s2rs::Session>) -> Result<Output, Output> {
    let this = session.user(name.as_str());

    match command {
        UserCommand::Comment { content, to, parent } => {
            this.send_comment(content).await.output_err()?;
            Ok(format![
                "Leaving a comment on {}'s profile",
                name.yellow(),
            ].into())
        },

        UserCommand::Follow => {
            this.follow().await.output_err()?;
            Ok(format![
                "Following {}",
                name.yellow()
            ].into())
        },

        UserCommand::Unfollow => {
            this.unfollow().await.output_err()?;
            Ok(format![
                "Unfollowing {}",
                name.yellow()
            ].into())
        }
    }
}