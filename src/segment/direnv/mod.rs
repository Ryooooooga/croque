use std::process::Command;

use super::{Context, Segment, SegmentBuilder};
use serde::Deserialize;

#[derive(Debug)]
pub struct DirenvSegmentBuilder {}

impl DirenvSegmentBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DirenvSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentBuilder for DirenvSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.direnv;
        let icons = &config.icons;

        let status: Status = load_direnv_status()?;
        let _ = status.state.found_rc?;

        let loaded_rc = status.state.loaded_rc.as_ref();

        let (icon, style) = match loaded_rc {
            Some(loaded_rc) if loaded_rc.allowed == DIRENV_ALLOWED => {
                (&icons.loaded, &config.loaded.style)
            }
            Some(loaded_rc) if loaded_rc.allowed == DIRENV_DENIED => {
                (&icons.disallowed, &config.disallowed.style)
            }
            _ => (&icons.found, &config.found.style),
        };

        let content = config.content.replace("{{.status}}", icon);

        Some(Segment {
            content,
            style: style.to_ansi(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct Status {
    // config: Config,
    state: State,
}

#[derive(Debug, Deserialize)]
struct State {
    #[serde(rename = "foundRC")]
    found_rc: Option<RC>,
    #[serde(rename = "loadedRC")]
    loaded_rc: Option<RC>,
}

#[derive(Debug, Deserialize)]
struct RC {
    allowed: i32,
    // path: String,
}

const DIRENV_ALLOWED: i32 = 0;
// const DIRENV_NOT_ALLOWED: i32 = 1;
const DIRENV_DENIED: i32 = 2;

fn load_direnv_status() -> Option<Status> {
    let output = Command::new("direnv")
        .args(["status", "--json"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let status: Status = serde_json::from_slice(&output.stdout).ok()?;

    Some(status)
}

#[cfg(test)]
mod tests {
    use crate::{command::SegmentArgs, config::Config, shell::Shell};

    use super::*;

    #[test]
    fn test_build() {
        let config = Config::default();

        struct Scenario {
            expected: Option<Segment>,
        }

        let scenarios: &[Scenario] = &[];

        for s in scenarios.iter() {
            let args = &SegmentArgs {
                right: false,
                exit_status: 0,
                duration: 0.0,
                jobs: 0,
                width: 100,
                encoded_git_info: None,
                encoded_gh_info: None,
                encoded_glab_info: None,
                shell: Shell::Zsh,
            };
            let ctx = Context::new(&config, args, None, None, None);

            let target = DirenvSegmentBuilder::new();

            let actual = target.build(&ctx);

            assert_eq!(actual, s.expected);
        }
    }
}
