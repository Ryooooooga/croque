use crate::{
    command::SegmentArgs,
    config::Config,
    info::{
        self,
        gh::{load_gh_info, GhInfo},
        git::{load_git_info, GitInfo},
        glab::{load_glab_info, GlabInfo},
    },
    segment::{self, Context},
};

fn decode_git_info(encoded_git_info: &str) -> Option<GitInfo> {
    if encoded_git_info.is_empty() {
        return None;
    }

    let bytes = info::decode_base64(encoded_git_info).ok()?;
    let (git_info, _): (GitInfo, usize) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).ok()?;

    Some(git_info)
}

fn decode_gh_info(encoded_gh_info: &str) -> Option<GhInfo> {
    if encoded_gh_info.is_empty() {
        return None;
    }

    let bytes = info::decode_base64(encoded_gh_info).ok()?;
    let (gh_info, _): (GhInfo, usize) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).ok()?;

    Some(gh_info)
}

fn decode_glab_info(encoded_glab_info: &str) -> Option<GlabInfo> {
    if encoded_glab_info.is_empty() {
        return None;
    }

    let bytes = info::decode_base64(encoded_glab_info).ok()?;
    let (glab_info, _): (GlabInfo, usize) =
        bincode::decode_from_slice(&bytes, bincode::config::standard()).ok()?;

    Some(glab_info)
}

pub fn run(args: &SegmentArgs) {
    let config = Config::load_or_default(Config::config_path());
    let git_info = match &args.encoded_git_info {
        Some(s) => decode_git_info(s),
        None => load_git_info(),
    };
    let gh_info = match &args.encoded_gh_info {
        Some(s) => decode_gh_info(s),
        None => load_gh_info(),
    };
    let glab_info = match &args.encoded_glab_info {
        Some(s) => decode_glab_info(s),
        None => load_glab_info(),
    };

    let ctx = Context::new(
        &config,
        args,
        git_info.as_ref(),
        gh_info.as_ref(),
        glab_info.as_ref(),
    );

    segment::print_segments(&ctx).unwrap();
}
