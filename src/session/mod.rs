use std::sync::Arc;
use colored::Colorize;
use rpassword::read_password as read_secret;
use crate::{input::{Input, Command, ProjectCommand, UserCommand}, store::{Store, Session}, output::{Output, OutputAsResult}};
use error::ResultUtils;

mod user;
mod project;
mod studio;
mod forum;
mod forum_post;
mod error;

pub async fn entry(input: Input, store: Store, session: Arc<s2rs::Session>) -> Result<Output, Output> {
    match input.command {
        Command::Auth { name } => {
            println!("Your scratch {}: (Character casing {} be correct)",
            "username".yellow(),
            "must".bold());
            let scratch_name = read_secret().output_err()?;

            println!("{}:",
            "X-Token".yellow());
            let x = read_secret().output_err()?;

            println!("{}:",
            "SessionID-Token".yellow());
            let session = read_secret().output_err()?;

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
            let scratch_name = match read_secret()

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

            let v = format!(
                "{} switched session to {}.",
                "Successfully".green(),
                (&to).yellow(),
            );

            panic![  ];
        },

        Command::User { name, command } => {
            user::entry(name, command, store, session).await
        },

        Command::Project { id, command } => {
            project::entry(id, command, store, session).await
        },

        Command::Studio { id, command } => {
            studio::entry(id, command, store, session).await
        },

        Command::Forum { id, command } => {
            forum::entry(id, command, store, session).await
        },

        Command::ForumPost { id, command } => {
            forum_post::entry(id, command, store, session).await
        },

        Command::Follow { name } => {
            user::entry(name, UserCommand::Follow, store, session).await
        },

        Command::Unfollow { name } => {
            user::entry(name, UserCommand::Unfollow, store, session).await
        },

        Command::Love { id } => {
            project::entry(id, ProjectCommand::Love, store, session).await
        }

        Command::Fav { id } => {
            project::entry(id, ProjectCommand::Fav, store, session).await
        },

        Command::Unfav { id } => {
            project::entry(id, ProjectCommand::Unfav, store, session).await
        },

        Command::Unlove { id } => {
            project::entry(id, ProjectCommand::Unlove, store, session).await
        }
    }
}