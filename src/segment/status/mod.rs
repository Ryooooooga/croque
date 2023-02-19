use super::{Context, Segment, SegmentBuilder};
use aho_corasick::AhoCorasick;
use std::borrow::Cow;

#[cfg(target_os = "windows")]
fn is_root() -> bool {
    false // TODO: for Windows
}

#[cfg(not(target_os = "windows"))]
fn is_root() -> bool {
    users::get_current_uid() == 0
}

pub struct StatusSegmentBuilder<'a> {
    replacer: AhoCorasick,
    is_root: &'a dyn Fn() -> bool,
}

impl<'a> StatusSegmentBuilder<'a> {
    pub fn new(is_root: &'a dyn Fn() -> bool) -> Self {
        let replacer = AhoCorasick::new(["{{.exit_status}}", "{{.root}}", "{{.jobs}}"]);
        Self { replacer, is_root }
    }
}

impl Default for StatusSegmentBuilder<'_> {
    fn default() -> Self {
        Self::new(&is_root)
    }
}

impl SegmentBuilder for StatusSegmentBuilder<'_> {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.status;
        let icons = &config.icons;
        let args = &ctx.args;

        let exit_status = if args.exit_status == 0 {
            Cow::from(&icons.succeeded)
        } else if config.failed.display_exit_code {
            Cow::from(format!("{} {}", icons.failed, args.exit_status))
        } else {
            Cow::from(&icons.failed)
        };

        let root = if (self.is_root)() {
            Cow::from(format!(" {}", icons.root))
        } else {
            Cow::from("")
        };

        let jobs = if args.jobs > 0 {
            Cow::from(format!(" {}", icons.jobs))
        } else {
            Cow::from("")
        };

        let style = if args.exit_status == 0 {
            &config.succeeded.style
        } else {
            &config.failed.style
        };

        let content = self
            .replacer
            .replace_all(&config.content, &[exit_status, root, jobs]);

        Some(Segment {
            content,
            style: style.to_ansi(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{command::SegmentArgs, config::Config, shell::Shell};

    use super::*;

    #[test]
    fn test_build() {
        let mut config = Config::default();

        struct Scenario {
            exit_status: i32,
            is_root: bool,
            jobs: i32,
            display_exit_code: bool,
            expected: Option<Segment>,
        }

        let scenarios = &[
            Scenario {
                exit_status: 0,
                is_root: false,
                jobs: 0,
                display_exit_code: true,
                expected: Some(Segment {
                    content: " ✓ ".to_string(),
                    style: config.status.succeeded.style.to_ansi(),
                }),
            },
            Scenario {
                exit_status: 1,
                is_root: false,
                jobs: 0,
                display_exit_code: true,
                expected: Some(Segment {
                    content: "  1 ".to_string(),
                    style: config.status.failed.style.to_ansi(),
                }),
            },
            Scenario {
                exit_status: 1,
                is_root: false,
                jobs: 0,
                display_exit_code: false,
                expected: Some(Segment {
                    content: "  ".to_string(),
                    style: config.status.failed.style.to_ansi(),
                }),
            },
            Scenario {
                exit_status: 130,
                is_root: true,
                jobs: 1,
                display_exit_code: true,
                expected: Some(Segment {
                    content: "  130   ".to_string(),
                    style: config.status.failed.style.to_ansi(),
                }),
            },
        ];

        for s in scenarios.iter() {
            config.status.failed.display_exit_code = s.display_exit_code;
            let args = &SegmentArgs {
                right: false,
                exit_status: s.exit_status,
                duration: 0.0,
                jobs: s.jobs,
                width: 100,
                encoded_git_info: None,
                encoded_gh_info: None,
                shell: Shell::Zsh,
            };
            let ctx = Context::new(&config, args, None, None);

            let is_root = || s.is_root;
            let target = StatusSegmentBuilder::new(&is_root);

            let actual = target.build(&ctx);

            assert_eq!(actual, s.expected);
        }
    }
}
