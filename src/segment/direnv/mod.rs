use super::{Context, Segment, SegmentBuilder};

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

        None
    }
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
