use crate::config::Config;

use super::{Segment, SegmentBuilder, SegmentError};

#[cfg(target_os = "windows")]
mod users {
    pub fn get_current_username() -> Option<std::ffi::OsString> {
        std::env::var_os("USERNAME")
    }
}

fn username() -> Option<String> {
    users::get_current_username().map(|username| username.to_string_lossy().to_string())
}

fn hostname() -> Option<String> {
    hostname::get()
        .ok()
        .map(|hostname| hostname.to_string_lossy().to_string())
}

pub struct UserSegmentBuilder<'a> {
    username: &'a dyn Fn() -> Option<String>,
    hostname: &'a dyn Fn() -> Option<String>,
}

impl Default for UserSegmentBuilder<'_> {
    fn default() -> Self {
        Self {
            username: &username,
            hostname: &hostname,
        }
    }
}

impl SegmentBuilder for UserSegmentBuilder<'_> {
    fn build(&self, config: &Config) -> Result<Option<Segment>, SegmentError> {
        let username = (self.username)().unwrap_or_else(|| String::from("?"));
        let hostname = (self.hostname)().unwrap_or_else(|| String::from("?"));

        Ok(Some(Segment {
            content: format!("{username}@{hostname}"),
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
            username: Option<&'static str>,
            hostname: Option<&'static str>,
            expected: Result<Option<Segment>, SegmentError>,
        }

        let scenarios = &[
            Scenario {
                testname: "should return segment",
                username: Some("user"),
                hostname: Some("host"),
                expected: Ok(Some(Segment {
                    content: "user@host".to_string(),
                })),
            },
            Scenario {
                testname: "should display `?` if username returns none",
                username: None,
                hostname: Some("host"),
                expected: Ok(Some(Segment {
                    content: "?@host".to_string(),
                })),
            },
            Scenario {
                testname: "should display `?` if hostname returns none",
                username: Some("user"),
                hostname: None,
                expected: Ok(Some(Segment {
                    content: "user@?".to_string(),
                })),
            },
        ];

        for s in scenarios {
            let config = &Config::default();

            let target = UserSegmentBuilder {
                username: &|| s.username.map(String::from),
                hostname: &|| s.hostname.map(String::from),
            };

            let actual = target.build(config);

            assert_eq!(actual, s.expected, "{}", s.testname);
        }
    }
}
