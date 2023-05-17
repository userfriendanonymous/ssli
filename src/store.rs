use std::{collections::HashMap, path::Path};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::{distributions::Alphanumeric, prelude::Distribution};
use tokio::{fs::{File, self}, io::{AsyncWriteExt, AsyncReadExt}};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use crate::{output::Output, session::SwitchError};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Sessions {
    pub items: HashMap<String, Session>
}

// region: Session
#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Session {
    pub x: String,
    pub session: String,
    pub name: String,
}

// impl From<cli::Auth> for Session {
//     fn from(value: cli::Auth) -> Self {
//         Self {
//             name: value.name,
//             session: value.session_token,
//             x: value.x_token
//         }
//     }
// }
// endregion: Session

// region: errors
#[derive(Debug, Clone)]
pub enum NewStoreError {
    Dirs,
}

#[derive(Debug)]
pub enum WriteError {
    Ser(serde_json::Error),
    Io(std::io::Error)
}

impl From<serde_json::Error> for WriteError {
    fn from(value: serde_json::Error) -> Self {
        Self::Ser(value)
    }
}

impl From<std::io::Error> for WriteError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<WriteError> for Output {
    fn from(value: WriteError) -> Self {
        match value {
            WriteError::Io(err) => Output::from("IO").with(err),
            WriteError::Ser(err) => Output::from("Serializing").with(err),
        }
    }
}
// endregion: errors

async fn write_file<P: AsRef<Path>>(path: P, name: &str, data: &[u8]) -> Result<usize, std::io::Error> {
    match File::open(&path).await {
        Ok(mut file) => {
            file.write(data).await
        }
        Err(_) => {
            tokio::fs::create_dir_all(&path).await?;
            let mut file = File::create(&path.as_ref().join(name)).await?;
            file.write(data).await
        }
    }
}

pub struct Store {
    dirs: ProjectDirs,
}

impl Store {
    fn dirs() -> Option<ProjectDirs> {
        directories::ProjectDirs::from("org", "UserFriend", "claw")
    }

    pub async fn key(&self) -> Option<String> {
        match File::open(self.dirs.config_dir().join("key")).await {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).await.unwrap();
                Some(data)
            }
            Err(_) => None
        }
    }

    pub async fn force_key(&self) -> String {
        match self.key().await {
            Some(key) => key,
            None => {
                let key: String = Alphanumeric.sample_iter(&mut rand::thread_rng()).take(16).map(char::from).collect();
                write_file(self.dirs.config_dir(), "key", key.as_bytes()).await.unwrap();
                key
            }
        }
    }

    pub fn new() -> Result<Self, NewStoreError> {
        let dirs = Self::dirs().ok_or(NewStoreError::Dirs)?;
        Ok(Self {
            dirs,
        })
    }

    pub async fn reset(&self) {
        if let Err(error) = fs::remove_dir_all(self.dirs.config_dir()).await {
            println!["failed to remove app `config`: {error}"];
        }
        if let Err(error) = fs::remove_dir_all(self.dirs.data_dir()).await {
            println!["failed to remove app `data`: {error}"];
        }
    }

    pub async fn decrypt_with_key(&self, data: String) -> String {
        let key = self.key().await.unwrap_or_default();
        let mc = new_magic_crypt!(key, 256);
         mc.decrypt_base64_to_string(data).unwrap_or_default()
    }

    pub async fn encrypt_with_key(&self, data: String) -> String {
        let key = self.force_key().await;
        let mc = new_magic_crypt!(key, 256);

        mc.encrypt_str_to_base64(data)
    }

    pub async fn sessions(&self) -> Sessions {
        match File::open(self.dirs.data_dir().join("sessions")).await {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).await.unwrap();

                match serde_json::from_str(&self.decrypt_with_key(data).await) {
                    Ok(data) => data,
                    Err(e) => {
                        println!["Error deserializing file contents: {e}"];
                        Sessions::default()
                    }
                }
            },
            Err(_) => {
                Sessions::default()
            }
        }
    }

    pub async fn main_session(&self) -> Session {
        match File::open(self.dirs.data_dir().join("main-session")).await {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).await.unwrap();

                match serde_json::from_str(&self.decrypt_with_key(data).await) {
                    Ok(data) => data,
                    Err(e) => {
                        println!["Error deserializing file contents: {e}"];
                        Session::default()
                    }
                }
            },
            Err(_) => {
                Session::default()
            }
        }
    }

    pub async fn remove_session(&self, name: &str) -> Result<(), SwitchError> {
        let mut sessions = self.sessions().await;
        let session = match sessions.items.remove(name) {
            Some(s) => s,
            None => Err(SwitchError::NotFound)?
        };

        if self.main_session().await.name == session.name {
            self.set_main_session(&Session::default()).await?;
        }

        write_file(self.dirs.data_dir(), "sessions", self.encrypt_with_key(
            serde_json::to_string(&sessions)?
        ).await.as_bytes()).await?;
        Ok(())
    }

    pub async fn add_session(&self, name: String, value: Session) -> Result<(), WriteError> {
        let mut sessions = self.sessions().await;
        sessions.items.insert(name, value);

        write_file(self.dirs.data_dir(), "sessions", self.encrypt_with_key(
            serde_json::to_string(&sessions)?
        ).await.as_bytes()).await?;
        Ok(())
    }

    pub async fn set_main_session(&self, value: &Session) -> Result<(), WriteError> {
        write_file(self.dirs.data_dir(), "main-session", self.encrypt_with_key(
            serde_json::to_string(&value)?
        ).await.as_bytes()).await?;
        Ok(())
    }
}