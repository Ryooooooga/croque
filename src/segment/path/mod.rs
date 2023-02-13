use super::{Context, Segment, SegmentBuilder};
use std::{borrow::Cow, path::PathBuf};

fn current_dir() -> Option<PathBuf> {
    std::env::current_dir()
        .ok()
        .or_else(|| std::env::var_os("PWD").map(PathBuf::from))
}

pub struct PathSegmentBuilder<'a> {
    current_dir: &'a dyn Fn() -> Option<PathBuf>,
}

impl Default for PathSegmentBuilder<'_> {
    fn default() -> Self {
        Self {
            current_dir: &current_dir,
        }
    }
}

impl SegmentBuilder for PathSegmentBuilder<'_> {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.path;

        let cwd = (self.current_dir)();
        let is_dir = cwd.as_ref().map(|cwd| cwd.is_dir()).unwrap_or(false);

        let style = if is_dir {
            &config.normal.style
        } else {
            &config.error.style
        };

        Some(Segment {
            content: format!(
                "{}",
                cwd.as_ref()
                    .map(|cwd| cwd.to_string_lossy())
                    .unwrap_or_else(|| Cow::from("<unknown>"))
            ),
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
        let config = &Config::default();
        let args = &SegmentArgs {
            exit_status: 0,
            duration: 0.0,
            jobs: 0,
            width: 100,
            shell: Shell::Zsh,
        };
        let ctx = Context::new(config, args);

        let cwd = std::env::current_dir().unwrap();
        let cwd = cwd.to_string_lossy();

        struct Scenario<'a> {
            testname: &'a str,
            cwd: Option<&'a str>,
            expected: Option<Segment>,
        }

        let scenarios = &[
            Scenario {
                testname: "should return segment if cwd is not none",
                cwd: Some(cwd.as_ref()),
                expected: Some(Segment {
                    content: cwd.to_string(),
                    style: config.path.normal.style.to_ansi(),
                }),
            },
            Scenario {
                testname: "should return <unknown> if cwd is none",
                cwd: None,
                expected: Some(Segment {
                    content: "<unknown>".to_string(),
                    style: config.path.error.style.to_ansi(),
                }),
            },
            Scenario {
                testname: "should return error style if cwd is not a directory",
                cwd: Some("NO_SUCH_DIR"),
                expected: Some(Segment {
                    content: "NO_SUCH_DIR".to_string(),
                    style: config.path.error.style.to_ansi(),
                }),
            },
        ];

        for s in scenarios {
            let target = PathSegmentBuilder {
                current_dir: &|| s.cwd.map(PathBuf::from),
            };

            let actual = target.build(&ctx);

            assert_eq!(actual, s.expected, "{}", s.testname);
        }
    }
}
