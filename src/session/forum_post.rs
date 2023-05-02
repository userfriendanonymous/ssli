use std::sync::Arc;
use colored::Colorize;
use crate::{input::ForumPostCommand, store::Store, output::Output};

pub async fn entry(id: u64, command: ForumPostCommand, store: Store, session: Arc<s2rs::Session>) -> Result<Output, Output> {
    match command {
    }
}