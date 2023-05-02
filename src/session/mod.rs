use std::sync::Arc;
use colored::Colorize;
use rpassword::read_password as read_secret;
use crate::{store::{Store, self}, output::Output};
use s2rs_derive::Forwarder;

mod user;
mod project;
mod studio;
mod forum;
mod forum_post;

pub struct Session {
    store: Arc<Store>,
    scratch: Arc<s2rs::Api>,
}

impl Session {
    pub fn new(store: Arc<Store>, scratch: Arc<s2rs::Api>) -> Self {
        Self {
            scratch,
            store
        }
    }
}

#[derive(Forwarder)]
pub enum IoStoreError {
    #[forward] Io(std::io::Error),
    #[forward] StoreWrite(store::WriteError)
}

impl From<IoStoreError> for Output {
    fn from(value: IoStoreError) -> Self {
        match value {
            IoStoreError::Io(err) => Output::from("IO").with(err),
            IoStoreError::StoreWrite(err) => Output::from("Writing to store").with(err)
        }
    }
}

#[derive(Forwarder)]
pub enum LoginError {
    #[forward(std::io::Error, store::WriteError)]
    IoStore(IoStoreError),
    #[forward] S2rs(s2rs::api::LoginError),
}

impl From<LoginError> for Output {
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::IoStore(err) => err.into(),
            LoginError::S2rs(err) => err.into()
        }
    }
}

#[derive(Forwarder)]
pub enum SwitchError {
    #[forward] StoreWrite(store::WriteError),
    NotFound
}

impl From<SwitchError> for Output {
    fn from(value: SwitchError) -> Self {
        match value {
            SwitchError::NotFound => "No auth found with such name".into(),
            SwitchError::StoreWrite(err) => Output::from("Writing store").with(err)
        }
    }
}

impl Session {
    pub async fn add_auth(&self, name: String) -> Result<(), IoStoreError> {
        println!("Your scratch {}: (Character casing {} be correct)",
        "username".yellow(),
        "must".bold());
        let scratch_name = read_secret()?;

        println!("{}:",
        "X-Token".yellow());
        let x = read_secret()?;

        println!("{}:",
        "SessionID-Token".yellow());
        let session = read_secret()?;

        self.store.add_session(name, store::Session {
            name: scratch_name,
            session,
            x
        }).await?;
        Ok(())
    }

    pub async fn login(&self, name: String) -> Result<(), LoginError> {
        println!("Your scratch {}: (Character casing {} be correct)",
        "username".yellow(),
        "must".bold());
        let scratch_name = read_secret()?;

        println!("{}:",
        "Password".yellow());
        let password = read_secret()?;

        let data = self.scratch.login(&scratch_name, &password).await?;

        let session = store::Session {
            name: scratch_name,
            session: data.session_token,
            x: data.x_token
        };

        self.store.add_session(name, session.clone()).await?;
        self.store.set_main_session(&session).await?;
        Ok(())
    }

    pub async fn switch(&self, to: String) -> Result<(), SwitchError> {
        let sessions = self.store.sessions().await;
        self.store.set_main_session(sessions.items.get(&to).ok_or(SwitchError::NotFound)?).await?;
        Ok(())
    }
    
}
