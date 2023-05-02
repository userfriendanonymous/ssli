use super::Session;

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

    pub async fn send_project_comment(&self, id: u64, content: &str, parent_id: Option<u64>, to_id: Option<u64>) -> s2rs::api::Result<()> {
        self.scratch.send_project_comment(id, content, parent_id, to_id).await
    }
}
