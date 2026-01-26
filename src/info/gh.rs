use std::{process::Command, thread};

use bitcode::{Decode, Encode};
use git2::Repository;
use serde::Deserialize;

#[derive(Debug, Encode, Decode)]
pub struct GhInfo {
    pub pull_request: Option<PullRequest>,
    pub actions: Option<Actions>,
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

#[derive(Debug, Encode, Decode)]
pub struct Actions {
    pub status: ActionsStatus,
}

#[derive(Debug, Encode, Decode)]
pub enum ActionsStatus {
    InProgress,
    Success,
    Failure,
    Cancelled,
    Skipped,
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

fn load_actions() -> Option<Actions> {
    let current_dir = std::env::current_dir().ok()?;
    let repo = Repository::discover(current_dir).ok()?;
    let head = repo.head().ok()?.target()?.to_string();

    let output = Command::new("gh")
        .args(["run", "list", "--commit", &head, "--json=conclusion"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    #[derive(Debug, Deserialize)]
    pub struct WorkflowRun {
        conclusion: Option<String>,
    }

    let runs: Vec<WorkflowRun> = serde_json::from_slice(&output.stdout).ok()?;

    fn get_actions_status(runs: &[WorkflowRun]) -> Option<ActionsStatus> {
        let mut in_progress: i32 = 0;
        let mut success: i32 = 0;
        let mut failure: i32 = 0;
        let mut cancelled: i32 = 0;
        let mut skipped: i32 = 0;

        for run in runs {
            match run.conclusion.as_deref() {
                None | Some("") => in_progress += 1,
                Some("success") => success += 1,
                Some("failure" | "timed_out") => failure += 1,
                Some("cancelled" | "stale") => cancelled += 1,
                Some("skipped" | "neutral") => skipped += 1,
                _ => {}
            }
        }

        if failure > 0 {
            Some(ActionsStatus::Failure)
        } else if in_progress > 0 {
            Some(ActionsStatus::InProgress)
        } else if cancelled > 0 {
            Some(ActionsStatus::Cancelled)
        } else if skipped > 0 {
            Some(ActionsStatus::Skipped)
        } else if success > 0 {
            Some(ActionsStatus::Success)
        } else {
            None
        }
    }

    let status = get_actions_status(&runs)?;
    Some(Actions { status })
}

pub fn load_gh_info() -> Option<GhInfo> {
    let pull_request_handle = thread::spawn(load_pull_request);
    let actions_handle = thread::spawn(load_actions);

    let pull_request = pull_request_handle.join().ok().flatten();
    let actions = actions_handle.join().ok().flatten();

    Some(GhInfo {
        pull_request,
        actions,
    })
}
