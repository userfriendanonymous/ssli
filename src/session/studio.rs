use super::Session;

impl Session {
    pub async fn follow_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.follow_studio(id).await
    }

    pub async fn unfollow_studio(&self, id: u64) -> s2rs::api::Result<()> {
        self.scratch.unfollow_studio(id).await
    }

    pub async fn send_studio_comment(&self, id: u64, content: &str, parent_id: Option<u64>, to_id: Option<u64>) -> s2rs::api::Result<()> {
        self.scratch.send_studio_comment(id, content, parent_id, to_id).await
    }
}
