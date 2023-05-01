use std::sync::Arc;
use colored::Colorize;
use crate::{input::ProjectCommand, store::Store, output::{Output, OutputErr}};

pub async fn entry(id: u64, command: ProjectCommand, store: Store, session: Arc<s2rs::Session>) -> Result<Output, Output> {
    let this = session.project(id);

    let result: Result<Output, Output> = match command {
        ProjectCommand::Comment { content, to, parent } => {
            this.send_comment(content.as_str()).await.output_err()?;
            Ok("Leaving a comment".into())
        },
        
        ProjectCommand::Fav => {
            this.love().await.output_err()?;
            Ok("Loving".into())
        },

        ProjectCommand::Love => {
            this.favorite().await.output_err()?;
            Ok("Favoriting".into())
        },

        ProjectCommand::Unlove => {
            this.unlove().await.output_err()?;
            Ok("Un-loving".into())
        },

        ProjectCommand::Unfav => {
            this.unfavorite().await.unwrap();
            Ok("Un-favoriting".into())
        },
    };
    
    Ok(Output::from(format![
        "On project #{}",
        id.to_string().yellow()
    ]).with(result?))
}