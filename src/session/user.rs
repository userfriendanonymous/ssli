use colored::Colorize;
use s2rs::api::{UserNameCheck, SendComment};
use crate::output::Output;
use super::Session;

impl From<UserNameCheck> for Output {
    fn from(value: UserNameCheck) -> Self {
        match value {
            UserNameCheck::Bad => "Bad".red().into(),
            UserNameCheck::Invalid => "Invalid".red().into(),
            UserNameCheck::Taken => "Taken".yellow().into(),
            UserNameCheck::Valid => "Available".green().into()
        }
    }
}

impl From<s2rs::api::User> for Output {
    fn from(value: s2rs::api::User) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Name: {}", value.name.yellow()]
        );
        output.push(
            format!["Id: {}", value.id.to_string().purple()]
        );
        output.push(
            format!["Scratch Team?: {}", value.scratch_team.to_string().blue()]
        );
        output.push(
            Output::from("History").with(value.history)
        );
        output.push(
            Output::from("Profile").with(value.profile)
        );
        output
    }
}

impl From<s2rs::api::UserHistory> for Output {
    fn from(value: s2rs::api::UserHistory) -> Self {
        let mut output = Output::from("");
        output.push(format![
            "Joined: {}", value.joined.cyan()
        ]);
        output
    }
}

impl From<s2rs::api::UserProfile> for Output {
    fn from(value: s2rs::api::UserProfile) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Id: {}", value.id.to_string().purple()]
        );
        output.push(
            format!["Country: {}", value.country.yellow()]
        );
        output.push(
            format!["Bio: {}", value.bio.cyan()]
        );
        output.push(
            format!["Wiwo: {}", value.status.cyan()]
        );
        output
    }
}

impl Session {
    pub async fn user_meta(&self, name: &str) -> s2rs::api::Result<s2rs::api::User> {
        self.scratch.user_meta(name).await
    }

    pub async fn follow_user(&self, name: &str) -> s2rs::api::Result<()> {
        self.scratch.follow_user(name).await?;
        Ok(())
    }

    pub async fn unfollow_user(&self, name: &str) -> s2rs::api::Result<()> {
        self.scratch.unfollow_user(name).await?;
        Ok(())
    }

    pub async fn send_user_comment(&self, name: &str, content: String, parent_id: Option<u64>, to_id: Option<u64>) -> s2rs::api::Result<()> {
        self.scratch.send_user_comment(name, &SendComment {
            content,
            parent_id,
            to_id
        }).await
    }

    pub async fn check_user(&self, name: &str) -> s2rs::api::Result<UserNameCheck> {
        self.scratch.check_user_name(name).await
    }
}
