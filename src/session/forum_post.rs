use std::sync::Arc;
use colored::Colorize;
use crate::{input::ForumPostCommand, store::Store};

pub async fn entry(id: u64, command: ForumPostCommand, store: Store, session: Arc<s2rs::Session>) {
    match command {
    }
}