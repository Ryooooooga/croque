use std::path::PathBuf;

use crate::config::Config;

use super::{Segment, SegmentBuilder, SegmentError};

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
    fn build(&self, config: &Config) -> Result<Option<Segment>, SegmentError> {
        let cwd = match (self.current_dir)() {
            Some(cwd) => cwd,
            None => return Ok(None),
        };

        Ok(Some(Segment {
            content: format!("{}", cwd.to_string_lossy(),),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        struct Scenario {
            testname: &'static str,
            cwd: Option<&'static str>,
            expected: Result<Option<Segment>, SegmentError>,
        }

        let scenarios = &[
            Scenario {
                testname: "should return segment if cwd is not none",
                cwd: Some("/home/john/Desktop"),
                expected: Ok(Some(Segment {
                    content: "/home/john/Desktop".to_string(),
                })),
            },
            Scenario {
                testname: "should return none if cwd is none",
                cwd: None,
                expected: Ok(None),
            },
        ];

        for s in scenarios {
            let config = &Config::default();

            let target = PathSegmentBuilder {
                current_dir: &|| s.cwd.map(PathBuf::from),
            };

            let actual = target.build(config);

            assert_eq!(actual, s.expected, "{}", s.testname);
        }
    }
}
