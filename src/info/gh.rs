use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GhInfo {
    pub pull_request: PullRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub number: i32,
    pub state: PullRequestState,
    pub comments: i32,
    pub is_draft: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

fn load_pull_request() -> Option<PullRequest> {
    let output = Command::new("gh")
        .args(["pr", "view", "--json=number,state,comments,isDraft"])
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
        state: PullRequestState,
        comments: Vec<PrComment>,
        #[serde(rename = "isDraft")]
        is_draft: bool,
    }

    let result: PrResult = serde_json::from_slice(&output.stdout).ok()?;
    let number = result.number;
    let state = result.state;
    let comments = result.comments.len() as i32;
    let is_draft = result.is_draft;

    Some(PullRequest {
        number,
        state,
        comments,
        is_draft,
    })
}

pub fn load_gh_info() -> Option<GhInfo> {
    let pull_request = load_pull_request()?;

    Some(GhInfo { pull_request })
}
