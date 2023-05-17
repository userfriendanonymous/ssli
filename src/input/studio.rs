use std::sync::Arc;
use clap::Subcommand;
use crate::{session::Session, output::Output};

#[derive(Debug, Subcommand)]
pub enum Studio {
    /// General metadata
    Meta,
    /// Send comment
    Cmt {
        content: String,
        #[arg(short, long)]
        parent: Option<u64>,
        #[arg(short, long)]
        to: Option<u64>,
    },
    /// Follow
    Fol,
    /// Unfollow
    Unf,
    /// Remove curator
    Kick { name: String },
    /// Invite curator
    Inv { name: String },
    /// Accept invite and join
    Join,
    /// Add project
    Add { project_id: u64 },
    /// Remove project
    Rem { project_id: u64 },
    /// Set description
    Desc { content: String },
    /// Set title
    Title { content: String },
    /// Toggle commenting
    Tgc,
    /// Promote curator to manager
    Pro { name: String },
    /// Open studio to public (this operation can't be done in Scratch website)
    Pub,
    /// Close studio from public (this operation can't be done in Scratch website)
    Lock,
}

impl Studio {
    pub async fn run(self, id: u64, session: Arc<Session>) -> Output {
        match self {
            Self::Meta => session.studio_meta(id).await.into(),
            Self::Fol => session.follow_studio(id).await.into(),
            Self::Unf => session.unfollow_studio(id).await.into(),
            Self::Kick { name } => session.remove_studio_curator(id, &name).await.into(),
            Self::Inv { name } => session.invite_studio_curator(id, &name).await.into(),
            Self::Join => session.accept_studio_invite(id).await.into(),
            Self::Cmt { content, parent, to } => session.send_studio_comment(id, content, parent, to).await.into(),
            Self::Add { project_id } => session.add_studio_project(id, project_id).await.into(),
            Self::Rem { project_id } => session.remove_studio_project(id, project_id).await.into(),
            Self::Desc { content } => session.set_studio_description(id, content).await.into(),
            Self::Title { content } => session.set_studio_title(id, content).await.into(),
            Self::Tgc => session.toggle_studio_commenting(id).await.into(),
            Self::Pro { name } => session.promote_studio_curator(id, &name).await.into(),
            Self::Pub => session.open_studio(id).await.into(),
            Self::Lock => session.close_studio(id).await.into(),
        }
    }
}
