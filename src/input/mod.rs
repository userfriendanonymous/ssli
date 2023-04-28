use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Input {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth { name: String },
    Login { name: String },
    Switch { to: String },

    User {
        name: String,
        #[clap(subcommand)]
        command: UserCommand
    },
    Project {
        id: u64,
        #[clap(subcommand)]
        command: ProjectCommand
    },
    Studio {
        id: u64,
        #[clap(subcommand)]
        command: StudioCommand
    },
    Forum {
        id: u64,
        #[clap(subcommand)]
        command: ForumCommand
    },
    ForumPost {
        id: u64,
        #[clap(subcommand)]
        command: ForumPostCommand
    },

    // aliases
    Follow { name: String },
    Unfollow { name: String },
    Love { id: u64 },
    Fav { id: u64 },
    Unlove { id: u64 },
    Unfav { id: u64 },
}

#[derive(Debug, Subcommand)]
pub enum UserCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Follow,
    Unfollow,
}

#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Love,
    Fav,
    Unlove,
    Unfav
}

#[derive(Debug, Subcommand)]
pub enum StudioCommand {
    Comment {
        content: String,
        #[arg(short, long)]
        to: Option<u64>,
        #[arg(short, long)]
        parent: Option<u64>,
    },
    Follow,
    Unfollow
}

#[derive(Debug, Subcommand)]
pub enum ForumCommand {
}


#[derive(Debug, Subcommand)]
pub enum ForumPostCommand {
}
