use bincode::{Decode, Encode};
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Encode, Decode)]
pub struct GlabInfo {
    pub merge_request: MergeRequest,
}

#[derive(Debug, Encode, Decode)]
pub struct MergeRequest {
    pub number: i32,
    pub state: MergeRequestState,
    pub comments: i32,
    pub pipeline: MrPipelineState,
    pub is_draft: bool,
    pub is_approved: bool,
}

#[derive(Debug, Encode, Decode)]
pub enum MergeRequestState {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Encode, Decode)]
pub enum MrPipelineState {
    None,
    Pending,
    Running,
    Success,
    Failed,
    Canceled,
}

fn load_merge_request() -> Option<MergeRequest> {
    let output = Command::new("glab")
        .args(["mr", "view", "--output=json", "--comments"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    #[derive(Debug, Deserialize)]
    struct MrComment {
        body: String,
        system: bool,
    }

    #[derive(Debug, Deserialize)]
    struct MrPipeline {
        status: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct MrResult {
        iid: i32,
        state: String,
        draft: bool,
        user_notes_count: i32,
        pipeline: Option<MrPipeline>,
        #[serde(rename = "Notes")]
        notes: Vec<MrComment>,
    }

    let result: MrResult = serde_json::from_slice(&output.stdout).ok()?;

    Some(MergeRequest {
        number: result.iid,
        state: match result.state.as_str() {
            "merged" => MergeRequestState::Merged,
            "closed" => MergeRequestState::Closed,
            _ => MergeRequestState::Open,
        },
        comments: result.user_notes_count,
        pipeline: result
            .pipeline
            .as_ref()
            .map(|pineline| match pineline.status.as_str() {
                "running" => MrPipelineState::Running,
                "success" => MrPipelineState::Success,
                "failed" => MrPipelineState::Failed,
                "canceled" => MrPipelineState::Canceled,
                _ => MrPipelineState::Pending,
            })
            .unwrap_or(MrPipelineState::None),
        is_draft: result.draft,
        is_approved: result
            .notes
            .iter()
            .any(|note| note.system && note.body == "approved this merge request"),
    })
}

pub fn load_glab_info() -> Option<GlabInfo> {
    let merge_request = load_merge_request()?;

    Some(GlabInfo { merge_request })
}
