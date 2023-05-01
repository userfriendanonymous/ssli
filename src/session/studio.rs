use std::sync::Arc;
use colored::Colorize;
use crate::{input::StudioCommand, store::Store, output::{Output, OutputErr}};

pub async fn entry(id: u64, command: StudioCommand, store: Store, session: Arc<s2rs::Session>) -> Result<Output, Output> {
    let this = session.studio(id);

    let result: Result<Output, Output> = match command {
        StudioCommand::Comment { content, to, parent } => {
            this.send_comment(&content).await.output_err()?;
            Ok("Leaving a comment".into())
        },

        StudioCommand::Follow => {
            this.follow().await.output_err()?;
            Ok("Following".into())
        },

        StudioCommand::Unfollow => {
            this.unfollow().await.output_err()?;
            Ok("Un-following".into())
        }
    };

    Ok(Output::from(format![
        "On studio #{}",
        id.to_string().yellow()
    ]).with(result?))
}