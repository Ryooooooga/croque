mod shrink;

use self::shrink::shrink_path;
use super::{Context, Segment, SegmentBuilder};
use std::path::PathBuf;

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

        let shrinked_path = cwd
            .map(|cwd| shrink_path(&cwd, config.shrink.enabled, config.shrink.length))
            .unwrap_or_else(|| String::from("<unknown>"));

        let content = config.content.replace("{{.path}}", &shrinked_path);

        let style = if is_dir {
            &config.normal.style
        } else {
            &config.error.style
        };

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
        let config = &Config::default();
        let args = &SegmentArgs {
            exit_status: 0,
            duration: 0.0,
            jobs: 0,
            width: 100,
            shell: Shell::Zsh,
        };
        let ctx = Context::new(config, args);

        struct Scenario<'a> {
            testname: &'a str,
            cwd: Option<&'a str>,
            expected_content: &'a str,
        }

        let scenarios = &[
            Scenario {
                testname: "should return segment if cwd is not none",
                cwd: Some("/home/ayaka/repos/github.com/Ryooooooga/croque/src"),
                expected_content: " /h/a/r/g/R/c/src ",
            },
            Scenario {
                testname: "should return <unknown> if cwd is none",
                cwd: None,
                expected_content: " <unknown> ",
            },
        ];

        for s in scenarios {
            let target = PathSegmentBuilder {
                current_dir: &|| s.cwd.map(PathBuf::from),
            };

            let actual = target.build(&ctx).unwrap();

            assert_eq!(&actual.content, s.expected_content, "{}", s.testname);
        }
    }
}
