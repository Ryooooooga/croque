use git2::{Reference, Repository, Status, StatusOptions};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct GitInfo {
    pub workdir: Option<PathBuf>,
    pub head: Head,
    pub workspace: WorkspaceStatus,
    pub remote: Option<RemoteStatus>,
    pub user: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum Head {
    Branch(String),
    Tag(String),
    Commit(String),
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceStatus {
    unstaged_new: u32,
    unstaged_deleted: u32,
    unstaged_changed: u32,
    unstaged_renamed: u32,
    staged_new: u32,
    staged_deleted: u32,
    staged_changed: u32,
    staged_renamed: u32,
    conflicted: u32,
}

impl WorkspaceStatus {
    pub fn has_new(&self) -> bool {
        self.unstaged_new > 0 || self.staged_new > 0
    }

    pub fn has_deleted(&self) -> bool {
        self.unstaged_deleted > 0 || self.staged_deleted > 0
    }

    pub fn has_modified(&self) -> bool {
        self.unstaged_changed > 0 || self.staged_changed > 0
    }

    pub fn has_renamed(&self) -> bool {
        self.unstaged_renamed > 0 || self.staged_renamed > 0
    }

    pub fn has_conflict(&self) -> bool {
        self.conflicted > 0
    }

    pub fn has_unstaged_changes(&self) -> bool {
        self.unstaged_new > 0
            || self.unstaged_deleted > 0
            || self.unstaged_changed > 0
            || self.unstaged_renamed > 0
    }

    pub fn has_staged_changes(&self) -> bool {
        self.staged_new > 0
            || self.staged_deleted > 0
            || self.staged_changed > 0
            || self.staged_renamed > 0
    }
}

#[derive(Debug, Deserialize)]
pub struct RemoteStatus {
    pub ahead: u32,
    pub behind: u32,
}

fn head_status(repo: &Repository, head_ref: &Option<Reference>) -> Head {
    let head_ref = match head_ref {
        Some(head_ref) => head_ref,
        None => {
            let config = repo.config().ok();
            let default_branch = config
                .as_ref()
                .and_then(|config| config.get_string("init.defaultBranch").ok())
                .unwrap_or_else(|| "master".to_string());

            return Head::Branch(default_branch);
        }
    };

    if head_ref.is_branch() {
        let branch_name = head_ref.shorthand().unwrap_or("?").to_string();
        return Head::Branch(branch_name);
    };

    let oid = match head_ref.target() {
        Some(oid) => oid,
        None => return Head::Commit("?".to_string()), // Because WTF?
    };

    if let Ok(tag) = repo.find_tag(oid) {
        return Head::Tag(tag.name().unwrap_or("?").to_string());
    }

    Head::Commit(oid.to_string())
}

fn workspace_status(repo: &Repository) -> WorkspaceStatus {
    let mut status_options = StatusOptions::new();
    status_options
        .include_untracked(true)
        .renames_head_to_index(true)
        .renames_index_to_workdir(true)
        .renames_from_rewrites(true);

    let mut status = WorkspaceStatus {
        unstaged_new: 0,
        unstaged_deleted: 0,
        unstaged_changed: 0,
        unstaged_renamed: 0,
        staged_new: 0,
        staged_deleted: 0,
        staged_changed: 0,
        staged_renamed: 0,
        conflicted: 0,
    };

    if let Ok(statuses) = repo.statuses(Some(&mut status_options)) {
        for s in statuses.iter() {
            let s = s.status();

            if s.intersects(Status::WT_NEW) {
                status.unstaged_new += 1;
            }
            if s.intersects(Status::WT_DELETED) {
                status.unstaged_deleted += 1;
            }
            if s.intersects(Status::WT_MODIFIED | Status::WT_TYPECHANGE) {
                status.unstaged_changed += 1;
            }
            if s.intersects(Status::WT_RENAMED) {
                status.unstaged_renamed += 1;
            }
            if s.intersects(Status::INDEX_NEW) {
                status.staged_new += 1;
            }
            if s.intersects(Status::INDEX_DELETED) {
                status.staged_deleted += 1;
            }
            if s.intersects(Status::INDEX_MODIFIED | Status::INDEX_TYPECHANGE) {
                status.staged_changed += 1;
            }
            if s.intersects(Status::INDEX_RENAMED) {
                status.staged_renamed += 1;
            }
            if s.intersects(Status::CONFLICTED) {
                status.conflicted += 1;
            }
        }
    }

    status
}

fn remote_status(repo: &Repository, head_ref: &Reference) -> Option<RemoteStatus> {
    let branch_name = head_ref.shorthand()?;
    let local_branch = repo
        .find_branch(branch_name, git2::BranchType::Local)
        .ok()?;
    let upstream_branch = local_branch.upstream().ok()?;

    let local_oid = head_ref.target()?;
    let upstream_oid = upstream_branch.get().target()?;

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid).ok()?;

    Some(RemoteStatus {
        ahead: ahead as u32,
        behind: behind as u32,
    })
}

fn user_name(repo: &Repository) -> Option<String> {
    let config = repo.config().ok()?;
    config.get_string("user.name").ok()
}

pub fn load_git_info() -> Option<GitInfo> {
    let current_dir = std::env::current_dir().ok()?;
    let repo = Repository::discover(current_dir).ok()?;

    let workdir = repo.workdir().map(PathBuf::from);

    let head_ref = repo.head().ok();

    let head = head_status(&repo, &head_ref);
    let workspace = workspace_status(&repo);
    let remote = head_ref
        .as_ref()
        .and_then(|head_ref| remote_status(&repo, head_ref));
    let user = user_name(&repo);

    Some(GitInfo {
        workdir,
        head,
        workspace,
        remote,
        user,
    })
}
