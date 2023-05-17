use super::Session;

impl Session {
    pub async fn send_topic_post(&self, id: u64, content: &str) -> s2rs::api::Result<()> {
        self.scratch.send_forum_post(id, content).await
    }
}
