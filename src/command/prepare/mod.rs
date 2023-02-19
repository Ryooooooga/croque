use super::PrepareArgs;
use crate::{
    command::DataSource,
    info::{self, git::load_git_info, gh::load_gh_info},
};

fn prepare_git_info() -> Option<Vec<u8>> {
    let git_info = load_git_info()?;
    bincode::serialize(&git_info).ok()
}

fn prepare_gh_info() -> Option<Vec<u8>> {
    let gh_info = load_gh_info()?;
    bincode::serialize(&gh_info).ok()
}

pub fn run(args: &PrepareArgs) {
    let bytes = match args.source {
        DataSource::Git => prepare_git_info(),
        DataSource::Gh => prepare_gh_info(),
    };

    if let Some(bytes) = &bytes {
        let encoded_bytes = info::encode_base64(bytes);
        println!("{encoded_bytes}");
    }
}
