use super::{Context, Segment, SegmentBuilder};

fn format_duration(duration: f64) -> String {
    static MICRO_SEC: f64 = 1e-6;
    static MILLI_SEC: f64 = 1e-3;
    static SEC: f64 = 1.0;
    static MIN: f64 = 60.0 * SEC;
    static HOUR: f64 = 60.0 * MIN;

    if duration < MILLI_SEC {
        format!("{:.0}μs", duration / MICRO_SEC)
    } else if duration < 10.0 * MILLI_SEC {
        format!("{:.2}ms", duration / MILLI_SEC)
    } else if duration < 100.0 * MILLI_SEC {
        format!("{:.1}ms", duration / MILLI_SEC)
    } else if duration < SEC {
        format!("{:.0}ms", duration / MILLI_SEC)
    } else if duration < 10.0 * SEC {
        format!("{:.2}s", duration / SEC)
    } else if duration + 0.05 * SEC < MIN {
        format!("{:.1}s", duration / SEC)
    } else if duration + 0.5 * SEC < HOUR {
        let d = duration + 0.5 * SEC;
        let mins = (d / MIN).floor();
        let secs = ((d % MIN) / SEC).floor();
        format!("{:.0}m {:.0}s", mins, secs)
    } else {
        let d = duration + 0.5 * SEC;
        let hours = (d / HOUR).floor();
        let mins = ((d % HOUR) / MIN).floor();
        let secs = ((d % MIN) / SEC).floor();
        format!("{:.0}h {:.0}m {:.0}s", hours, mins, secs)
    }
}

#[derive(Debug, Default)]
pub struct DurationSegmentBuilder {}

impl SegmentBuilder for DurationSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.duration;
        let duration = ctx.args.duration;

        if duration <= 0.0 {
            return None;
        }

        let content = config
            .content
            .replace("{{.duration}}", &format_duration(duration));
        let style = config.style.to_ansi();

        Some(Segment { content, style })
    }
}

#[cfg(test)]
mod tests {
    use crate::{command::SegmentArgs, config::Config};

    use super::*;

    #[test]
    fn test_build() {
        struct Scenario {
            duration: f64,
            expected_content: Option<&'static str>,
        }

        let scenarios = &[
            Scenario {
                duration: 0.0,
                expected_content: None,
            },
            Scenario {
                duration: 1.414_213,
                expected_content: Some(" 祥1.41s "),
            },
            Scenario {
                duration: 2.236,
                expected_content: Some(" 祥2.24s "),
            },
            Scenario {
                duration: 0.567_8,
                expected_content: Some(" 祥568ms "),
            },
            Scenario {
                duration: 0.056_78,
                expected_content: Some(" 祥56.8ms "),
            },
            Scenario {
                duration: 0.005_678,
                expected_content: Some(" 祥5.68ms "),
            },
            Scenario {
                duration: 0.000_567_8,
                expected_content: Some(" 祥568μs "),
            },
            Scenario {
                duration: 0.000_056_78,
                expected_content: Some(" 祥57μs "),
            },
            Scenario {
                duration: 0.000_005_678,
                expected_content: Some(" 祥6μs "),
            },
            Scenario {
                duration: 0.000_000_512,
                expected_content: Some(" 祥1μs "),
            },
            Scenario {
                duration: 0.000_000_499,
                expected_content: Some(" 祥0μs "),
            },
            Scenario {
                duration: 59.94,
                expected_content: Some(" 祥59.9s "),
            },
            Scenario {
                duration: 59.95,
                expected_content: Some(" 祥1m 0s "),
            },
            Scenario {
                duration: 92.3,
                expected_content: Some(" 祥1m 32s "),
            },
            Scenario {
                duration: 3599.4,
                expected_content: Some(" 祥59m 59s "),
            },
            Scenario {
                duration: 3599.5,
                expected_content: Some(" 祥1h 0m 0s "),
            },
            Scenario {
                duration: 12.0 * 60.0 * 60.0 + 38.0 * 60.0 + 45.0,
                expected_content: Some(" 祥12h 38m 45s "),
            },
        ];

        let config = Config::default();
        for s in scenarios {
            let args = SegmentArgs {
                exit_status: 0,
                duration: s.duration,
                jobs: 0,
                width: 100,
                shell: crate::shell::Shell::Zsh,
            };

            let ctx = &Context::new(&config, &args, None);

            let target = DurationSegmentBuilder::default();
            let actual = target.build(ctx);
            let actual_content = actual.as_ref().map(|seg| seg.content.as_str());

            assert_eq!(
                actual_content, s.expected_content,
                "duration={}",
                s.duration
            );
        }
    }
}
