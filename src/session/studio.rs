use colored::Colorize;
use s2rs::api::{StudioInfo, SendComment};
use crate::output::Output;

use super::Session;

impl From<s2rs::api::Studio> for Output {
    fn from(value: s2rs::api::Studio) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Id: {}", value.id.to_string().purple()]
        );
        output.push(
            format!["Public?: {}", value.public.to_string().blue()]
        );
        output.push(
            format!["Open to all?: {}", value.open_to_all.to_string().blue()]
        );
        output.push(
            format!["Comments on?: {}", value.comments_allowed.to_string().blue()]
        );
        output.push(
            format!["Host user id: {}", value.host.to_string().purple()]
        );
        output.push(
            format!["Title: {}", value.title.yellow()]
        );
        output.push(
            format!["Description: {}", value.description.cyan()]
        );
        output.push(
            Output::from("Stats").with(value.stats)
        );
        output.push(
            Output::from("History").with(value.history)
        );
        output
    }
}

impl From<s2rs::api::StudioHistory> for Output {
    fn from(value: s2rs::api::StudioHistory) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Created: {}", value.created.cyan()]
        );
        output.push(
            format!["Modified: {}", value.modified.cyan()]
        );
        output
    }
}

impl From<s2rs::api::StudioStats> for Output {
    fn from(value: s2rs::api::StudioStats) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Projects: {}", value.projects.to_string().purple()]
        );
        output.push(
            format!["Followers: {}", value.followers.to_string().purple()]
        );
        output.push(
            format!["Comments: {}", value.comments.to_string().purple()]
        );
        output.push(
            format!["Managers: {}", value.managers.to_string().purple()]
        );
        output
    }
}

impl Session {
    pub async fn studio_meta(&self, id: u64) -> s2rs::api::Result<s2rs::api::Studio> {
        self.scratch.studio_meta(id).await
    }


    pub async fn follow_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.follow_studio(id).await
    }

    pub async fn unfollow_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.unfollow_studio(id).await
    }

    pub async fn send_studio_comment(&self, id: u64, content: String, parent_id: Option<u64>, to_id: Option<u64>) -> s2rs::api::Result<()> {
        self.scratch.send_studio_comment(id, &SendComment {
            content,
            parent_id,
            to_id
        }).await
    }

    pub async fn remove_studio_curator(&self, id: u64, name: &str) -> s2rs::api::Result<()> {
        self.scratch.remove_studio_curator(id, name).await
    }

    pub async fn invite_studio_curator(&self, id: u64, name: &str) -> s2rs::api::Result<()> {
        self.scratch.invite_studio_curator(id, name).await
    }

    pub async fn accept_studio_invite(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.accept_studio_invite(id).await
    }

    pub async fn add_studio_project(&self, id: u64, project_id: u64) -> s2rs::api::Result<()> {
        self.scratch.add_studio_project(id, project_id).await
    }

    pub async fn remove_studio_project(&self, id: u64, project_id: u64) -> s2rs::api::Result<()> {
        self.scratch.remove_studio_project(id, project_id).await
    }

    pub async fn set_studio_description(&self, id: u64, content: String) -> s2rs::api::Result<()> {
        self.scratch.set_studio_info(id, &StudioInfo {
            description: Some(content),
            ..Default::default()
        }).await
    }

    pub async fn set_studio_title(&self, id: u64, content: String) -> s2rs::api::Result<()> {
        self.scratch.set_studio_info(id, &StudioInfo {
            title: Some(content),
            ..Default::default()
        }).await
    }

    pub async fn toggle_studio_commenting(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.toggle_studio_commenting(id).await
    }

    pub async fn promote_studio_curator(&self, id: u64, name: &str) -> s2rs::api::Result<()> {
        self.scratch.promote_studio_curator(id, name).await
    }

    pub async fn open_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.open_studio(id).await
    }

    pub async fn close_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.close_studio(id).await
    }
}
