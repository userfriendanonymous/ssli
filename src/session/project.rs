use colored::Colorize;
use s2rs::api::SendComment;
use crate::output::Output;
use super::Session;

impl From<s2rs::api::Project> for Output {
    fn from(value: s2rs::api::Project) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Id: {}", value.id.to_string().purple()]
        );
        output.push(
            format!["Comments on?: {}", value.comments_allowed.to_string().blue()]
        );
        output.push(
            format!["Public?: {}", value.public.to_string().blue()]
        );
        output.push(
            format!["Title: {}", value.title.yellow()]
        );
        output.push(
            format!["Description: {}", value.description.cyan()]
        );
        output.push(
            format!["Instructions: {}", value.instructions.cyan()]
        );
        output.push(
            Output::from("Stats").with(value.stats)
        );
        output.push(
            Output::from("Author").with(value.author)
        );
        output
    }
}

impl From<s2rs::api::ProjectAuthor> for Output {
    fn from(value: s2rs::api::ProjectAuthor) -> Self {
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
        output
    }
}

impl From<s2rs::api::ProjectStats> for Output {
    fn from(value: s2rs::api::ProjectStats) -> Self {
        let mut output = Output::from("");
        output.push(
            format!["Loves: {}", value.loves.to_string().yellow()]
        );
        output.push(
            format!["Favorites: {}", value.favorites.to_string().yellow()]
        );
        output.push(
            format!["Remixes: {}", value.remixes.to_string().yellow()]
        );
        output.push(
            format!["Views: {}", value.views.to_string().yellow()]
        );
        output
    }
}

impl Session {
    pub async fn love_project(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.love_project(id).await
    }

    pub async fn unlove_project(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.unlove_project(id).await
    }

    pub async fn favorite_project(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.favorite_project(id).await
    }

    pub async fn unfavorite_project(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.unfavorite_project(id).await
    }

    pub async fn send_project_comment(&self, id: u64, content: String, parent_id: Option<u64>, to_id: Option<u64>) -> s2rs::api::Result<()> {
        self.scratch.send_project_comment(id, &SendComment {
            content,
            parent_id,
            to_id
        }).await
    }

    pub async fn project_meta(&self, id: u64) -> s2rs::api::Result<s2rs::api::Project> {
        self.scratch.project_meta(id).await
    }
}
