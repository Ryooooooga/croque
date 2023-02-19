use aho_corasick::AhoCorasick;

use super::{Context, Segment, SegmentBuilder};

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
    replacer: AhoCorasick,
    username: &'a dyn Fn() -> Option<String>,
    hostname: &'a dyn Fn() -> Option<String>,
}

impl<'a> UserSegmentBuilder<'a> {
    pub fn new(
        username: &'a dyn Fn() -> Option<String>,
        hostname: &'a dyn Fn() -> Option<String>,
    ) -> Self {
        let replacer = AhoCorasick::new(["{{.user}}", "{{.host}}"]);

        Self {
            replacer,
            username,
            hostname,
        }
    }
}

impl Default for UserSegmentBuilder<'_> {
    fn default() -> Self {
        Self::new(&username, &hostname)
    }
}

impl SegmentBuilder for UserSegmentBuilder<'_> {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.user;

        let user = (self.username)().unwrap_or_else(|| String::from("?"));
        let host = (self.hostname)().unwrap_or_else(|| String::from("?"));
        let content = self.replacer.replace_all(&config.content, &[&user, &host]);

        Some(Segment {
            content,
            style: config.style.to_ansi(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{command::SegmentArgs, config::Config, shell::Shell};

    use super::*;

    #[test]
    fn test_build() {
        struct Scenario {
            testname: &'static str,
            template: &'static str,
            username: Option<&'static str>,
            hostname: Option<&'static str>,
            expected_content: Option<&'static str>,
        }

        let scenarios = &[
            Scenario {
                testname: "should return segment",
                template: " {{.user}}@{{.host}} ",
                username: Some("user"),
                hostname: Some("host"),
                expected_content: Some(" user@host "),
            },
            Scenario {
                testname: "should display `?` if username returns none",
                template: " {{.user}}@{{.host}} ",
                username: None,
                hostname: Some("host"),
                expected_content: Some(" ?@host "),
            },
            Scenario {
                testname: "should display `?` if hostname returns none",
                template: " {{.user}}@{{.host}} ",
                username: Some("user"),
                hostname: None,
                expected_content: Some(" user@? "),
            },
        ];

        for s in scenarios {
            let mut config = Config::default();
            config.user.content = s.template.to_string();

            let args = &SegmentArgs {
                right: false,
                exit_status: 0,
                duration: 0.0,
                jobs: 0,
                width: 100,
                encoded_git_info: None,
                encoded_gh_info: None,
                shell: Shell::Zsh,
            };

            let ctx = Context::new(&config, args, None, None);

            let username = || s.username.map(String::from);
            let hostname = || s.hostname.map(String::from);
            let target = UserSegmentBuilder::new(&username, &hostname);

            let actual = target.build(&ctx);
            let actual_content = actual.as_ref().map(|seg| seg.content.as_str());

            assert_eq!(actual_content, s.expected_content, "{}", s.testname);
        }
    }
}
