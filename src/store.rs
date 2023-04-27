use std::{collections::HashMap, path::Path};
use tokio::{fs::File, io::{AsyncWriteExt, AsyncReadExt}};
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};

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

    pub fn new() -> Result<Self, NewStoreError> {
        let dirs = Self::dirs().ok_or(NewStoreError::Dirs)?;
        Ok(Self {
            dirs,
        })
    }

    pub async fn sessions(&self) -> Sessions {
        match File::open(self.dirs.data_dir().join("sessions")).await {
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).await;

                match serde_json::from_str(&data) {
                    Ok(data) => data,
                    Err(_) => {
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
                file.read_to_string(&mut data).await;

                match serde_json::from_str(&data) {
                    Ok(data) => data,
                    Err(e) => {
                        Session::default()
                    }
                }
            },
            Err(_) => Session::default()
        }
    }

    pub async fn add_session(&self, name: String, value: Session) -> Result<(), WriteError> {
        let mut sessions = self.sessions().await;
        sessions.items.insert(name, value);

        write_file(self.dirs.data_dir(), "sessions", &serde_json::to_vec(&sessions)?).await?;
        Ok(())
    }

    pub async fn set_main_session(&self, value: &Session) -> Result<(), WriteError> {
        write_file(self.dirs.data_dir(), "main-session", &serde_json::to_vec(value)?).await?;
        Ok(())
    }
}