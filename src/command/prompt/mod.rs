use crate::{
    command::SegmentArgs,
    config::Config,
    info::{
        self,
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

pub fn run(args: &SegmentArgs) {
    let config = &Config::default();
    let git_info = match &args.encoded_git_info {
        Some(s) => decode_git_info(s),
        None => load_git_info(),
    };
    let ctx = Context::new(config, args, git_info.as_ref());

    segment::print_segments(&ctx).unwrap();
}
