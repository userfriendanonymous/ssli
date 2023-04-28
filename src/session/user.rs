use std::sync::Arc;
use colored::Colorize;
use crate::{input::UserCommand, store::Store};

pub async fn entry(name: String, command: UserCommand, store: Store, session: Arc<s2rs::Session>) {
    let this = session.user(name.as_str());

    match command {
        UserCommand::Comment { content, to, parent } => {
            this.send_comment(content).await.unwrap();
            println!("{} left a comment on {}'s profile.",
                "Successfully".green(),
                name.yellow(),
            );
        },

        UserCommand::Follow => {
            this.follow().await.unwrap();
            println!("{} followed {}.", "Successfully".green(), (&name).yellow());
        },

        UserCommand::Unfollow => {
            this.unfollow().await.unwrap();
            println!("{} unfollowed {}.", "Successfully".green(), (&name).yellow());
        }
    }
}