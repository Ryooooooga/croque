use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GhInfo {
    pub pull_request: PullRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: i32,
    pub comments: i32,
    pub is_closed: bool,
    pub is_draft: bool,
}

fn load_pull_request() -> Option<PullRequest> {
    let output = Command::new("gh")
        .args(&["pr", "view", "--json=number,comments,closed,isDraft"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    #[derive(Debug, Deserialize)]
    struct PrComment {}

    #[derive(Debug, Deserialize)]
    struct PrResult {
        number: i32,
        comments: Vec<PrComment>,
        closed: bool,
        #[serde(rename = "isDraft")]
        is_draft: bool,
    }

    let result: PrResult = serde_json::from_slice(&output.stdout).ok()?;
    let number = result.number;
    let comments = result.comments.len() as i32;
    let is_closed = result.closed;
    let is_draft = result.is_draft;

    Some(PullRequest {
        number,
        comments,
        is_closed,
        is_draft,
    })
}

pub fn load_gh_info() -> Option<GhInfo> {
    let pull_request = load_pull_request()?;

    Some(GhInfo { pull_request })
}
