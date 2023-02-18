use super::PrepareArgs;
use crate::{
    command::DataSource,
    info::{self, git::load_git_info},
};

fn prepare_git_info() -> Option<Vec<u8>> {
    let git_info = load_git_info()?;
    bincode::serialize(&git_info).ok()
}

pub fn run(args: &PrepareArgs) {
    let bytes = match args.source {
        DataSource::Git => prepare_git_info(),
    };

    if let Some(bytes) = &bytes {
        let encoded_bytes = info::encode_base64(bytes);
        println!("{encoded_bytes}");
    }
}
