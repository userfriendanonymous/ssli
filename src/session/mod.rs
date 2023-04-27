use std::sync::Arc;

use colored::Colorize;
use rpassword::read_password as read_secret;
use crate::{input::{Input, Command, UserCommand}, store::{Store, Session}};

pub async fn entry(input: Input, store: Store, session: Arc<s2rs::Session>) {
    match input.command {
        Command::Auth { name } => {
            println!("Your scratch {}: (Character casing {} be correct)",
            "username".yellow(),
            "must".bold());
            let scratch_name = read_secret().unwrap();

            println!("{}:",
            "X-Token".yellow());
            let x = read_secret().unwrap();

            println!("{}:",
            "SessionID-Token".yellow());
            let session = read_secret().unwrap();

            store.add_session(name, Session {
                name: scratch_name,
                session,
                x
            }).await.unwrap();

            println!("{} added session.", "Successfully".green());
        },

        Command::Login { name } => {
            println!("Your scratch {}: (Character casing {} be correct)",
            "username".yellow(),
            "must".bold());
            let scratch_name = read_secret().unwrap();

            println!("{}:",
            "Password".yellow());
            let password = read_secret().unwrap();

            let data = session.me().login(&scratch_name, &password).await.unwrap();

            let session = Session {
                name: scratch_name,
                session: data.session_token,
                x: data.x_token
            };

            store.add_session(name, session.clone()).await.unwrap();
            store.set_main_session(&session).await.unwrap();

            println!("{} logged in.", "Successfully".green());
        }

        Command::Switch { to } => {
            let sessions = store.sessions().await;
            store.set_main_session(sessions.items.get(&to).unwrap()).await.unwrap();

            println!("{} switched session to {}.",
                "Successfully".green(),
                (&to).yellow(),
            );
        },

        Command::User { name, command } => {
            match command {
                UserCommand::Comment { content, to, parent } => {
                    session.user(name.as_str()).send_comment(content).await.unwrap();
                    println!("{} left a comment on {}'s profile.",
                        "Successfully".green(),
                        name.yellow(),
                    );
                }
            }
        }

        Command::Follow { name } => {
            session.user(name.as_str()).follow().await.unwrap();
            println!("{} followed {}.", "Successfully".green(), (&name).yellow());
        },

        Command::Unfollow { name } => {
            session.user(name.as_str()).unfollow().await.unwrap();
            println!("{} unfollowed {}.", "Successfully".green(), (&name).yellow());
        },

        Command::Love { id } => {
            session.project(id).love().await.unwrap();
            println!("{} loved {}.",
                "Successfully".green(),
                id.to_string().yellow()
            );
        }

        Command::Fav { id } => {
            session.project(id).favorite().await.unwrap();
            println!("{} favorited {}.",
                "Successfully".green(),
                id.to_string().yellow()
            );
        },
    }
}