use std::sync::Arc;
use colored::Colorize;
use crate::{input::ForumCommand, store::Store};

pub async fn entry(id: u64, command: ForumCommand, store: Store, session: Arc<s2rs::Session>) {
    match command {
    }
}