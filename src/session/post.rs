use super::Session;

impl Session {
    pub async fn post_content(&self, id: u64) -> s2rs::api::Result<String> {
        self.scratch.forum_post_content(id).await
    }

    pub async fn edit_post(&self, id: u64, content: &str) -> s2rs::api::Result<()> {
        self.scratch.edit_forum_post(id, content).await
    }
}
