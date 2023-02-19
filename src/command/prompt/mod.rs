use crate::{
    command::SegmentArgs,
    config::Config,
    info::{
        self,
        gh::{load_gh_info, GhInfo},
        git::{load_git_info, GitInfo},
    },
    segment::{self, Context},
};

fn decode_git_info(encoded_git_info: &str) -> Option<GitInfo> {
    if encoded_git_info.is_empty() {
        return None;
    }

    let bytes = info::decode_base64(encoded_git_info).ok()?;
    let git_info: GitInfo = bincode::deserialize::<GitInfo>(&bytes).ok()?;

    Some(git_info)
}

fn decode_gh_info(encoded_gh_info: &str) -> Option<GhInfo> {
    if encoded_gh_info.is_empty() {
        return None;
    }

    let bytes = info::decode_base64(encoded_gh_info).ok()?;
    let gh_info: GhInfo = bincode::deserialize::<GhInfo>(&bytes).ok()?;

    Some(gh_info)
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

    let ctx = Context::new(&config, args, git_info.as_ref(), gh_info.as_ref());

    segment::print_segments(&ctx).unwrap();
}
