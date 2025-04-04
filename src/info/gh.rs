use std::process::Command;

use bincode::{Decode, Encode};
use serde::Deserialize;

#[derive(Debug, Encode, Decode)]
pub struct GhInfo {
    pub pull_request: PullRequest,
}

#[derive(Debug, Encode, Decode)]
pub struct PullRequest {
    pub number: i32,
    pub state: PullRequestState,
    pub comments: i32,
    pub is_draft: bool,
    pub is_approved: bool,
}

#[derive(Debug, Encode, Decode, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

fn load_pull_request() -> Option<PullRequest> {
    let output = Command::new("gh")
        .args([
            "pr",
            "view",
            "--json=number,state,comments,reviews,reviewDecision,isDraft",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    #[derive(Debug, Deserialize)]
    struct PrComment {}

    #[derive(Debug, Deserialize)]
    struct PrReview {}

    #[derive(Debug, Deserialize)]
    struct PrResult {
        number: i32,
        state: PullRequestState,
        comments: Vec<PrComment>,
        reviews: Vec<PrReview>,
        #[serde(rename = "reviewDecision")]
        review_decision: String,
        #[serde(rename = "isDraft")]
        is_draft: bool,
    }

    let result: PrResult = serde_json::from_slice(&output.stdout).ok()?;
    let number = result.number;
    let state = result.state;
    let comments = (result.comments.len() + result.reviews.len()) as i32;
    let is_draft = result.is_draft;
    let is_approved = result.review_decision == "APPROVED";

    Some(PullRequest {
        number,
        state,
        comments,
        is_draft,
        is_approved,
    })
}

pub fn load_gh_info() -> Option<GhInfo> {
    let pull_request = load_pull_request()?;

    Some(GhInfo { pull_request })
}
