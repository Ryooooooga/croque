use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlabInfo {
    pub merge_request: MergeRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    pub number: i32,
    pub state: MergeRequestState,
    pub comments: i32,
    pub is_draft: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MergeRequestState {
    Open,
    Closed,
    Merged,
}

fn load_merge_request() -> Option<MergeRequest> {
    let output = Command::new("glab").args(["mr", "view"]).output().ok()?;

    if !output.status.success() {
        return None;
    }

    let mut mr = MergeRequest {
        number: 0,
        state: MergeRequestState::Open,
        comments: 0,
        is_draft: false,
    };

    let stdout = std::str::from_utf8(&output.stdout).ok()?;
    for line in stdout.split('\n') {
        let trimmed_line = line.trim();
        if let Some(number) = trimmed_line.strip_prefix("number:") {
            mr.number = number.trim().parse().unwrap_or(0);
        } else if let Some(state) = trimmed_line.strip_prefix("state:") {
            mr.state = match state.trim() {
                "merged" => MergeRequestState::Merged,
                "closed" => MergeRequestState::Closed,
                _ => MergeRequestState::Open,
            };
        } else if let Some(title) = trimmed_line.strip_prefix("title:") {
            mr.is_draft = title.trim().starts_with("Draft:");
        } else if let Some(comments) = trimmed_line.strip_prefix("comments:") {
            mr.comments = comments.trim().parse().unwrap_or(0);
        } else if trimmed_line == "--" {
            break;
        }
    }

    if mr.number == 0 {
        return None;
    }

    Some(mr)
}

pub fn load_glab_info() -> Option<GlabInfo> {
    let merge_request = load_merge_request()?;

    Some(GlabInfo { merge_request })
}
