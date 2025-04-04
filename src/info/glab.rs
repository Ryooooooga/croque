use bincode::{Decode, Encode};
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[derive(Debug, Encode, Decode)]
pub struct GlabInfo {
    pub merge_request: MergeRequest,
}

#[derive(Debug, Encode, Decode)]
pub struct MergeRequest {
    pub number: i32,
    pub state: MergeRequestState,
    pub comments: i32,
    pub is_draft: bool,
}

#[derive(Debug, Encode, Decode)]
pub enum MergeRequestState {
    Open,
    Closed,
    Merged,
}

fn load_merge_request() -> Option<MergeRequest> {
    let mut process = Command::new("glab")
        .args(["mr", "view"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = process.stdin.take() {
        let _ = stdin.write_all(b"\n");
    }

    let output = process.wait_with_output().ok()?;

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
            mr.is_draft = title.trim().to_lowercase().starts_with("draft:");
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
