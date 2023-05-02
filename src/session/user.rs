use super::Session;

impl Session {
    pub async fn follow_user(&self, name: String) -> Result<(), s2rs::api::Error> {
        self.scratch.user(name).follow().await
    }

    pub async fn unfollow_user(&self, name: String) -> Result<(), s2rs::api::Error> {
        self.scratch.user(name).unfollow().await
    }

    pub async fn comment_user(&self, name: String, content: String, to: Option<u64>, parent: Option<u64>) -> Result<(), s2rs::api::Error> {
        self.scratch.user(name).send_comment(content).await
    }
}