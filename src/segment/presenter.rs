use crate::{config::Config, shell::Shell};
use ansi_term::{Colour, Style};
use std::io;

use super::Segment;

#[derive(Debug)]
pub struct Presenter<'a, W: io::Write> {
    out: &'a mut W,
    config: &'a Config,
    shell: &'a Shell,

    prev_bg: Option<Colour>,
}

impl<'a, W: io::Write> Presenter<'a, W> {
    pub fn new(out: &'a mut W, config: &'a Config, shell: &'a Shell) -> Self {
        Self {
            out,
            config,
            shell,
            prev_bg: None,
        }
    }

    fn display_segment(&mut self, segment: &Segment) -> io::Result<()> {
        let prefix = self.shell.control(segment.style.prefix());
        let suffix = self.shell.control(segment.style.suffix());
        let content = self.shell.escape(&segment.content);

        write!(self.out, "{prefix}{content}{suffix}")
    }

    fn display_left_closure(&mut self) -> io::Result<()> {
        if let Some(prev_bg) = self.prev_bg {
            let style = Style::new().fg(prev_bg);
            let prefix = self.shell.control(style.prefix());
            let suffix = self.shell.control(style.suffix());
            let content = &self.config.segment_separators.solid_left;
            write!(self.out, "{prefix}{content}{suffix}")
        } else {
            let style = Style::new().fg(Colour::White);
            let prefix = self.shell.control(style.prefix());
            let suffix = self.shell.control(style.suffix());
            let content = &self.config.segment_separators.wire_left;
            write!(self.out, "{prefix}{content}{suffix}")
        }
    }

    fn display_left_separator(&mut self, next_bg: Option<Colour>) -> io::Result<()> {
        if next_bg == self.prev_bg {
            let style = Style::new().fg(Colour::White);
            let prefix = self.shell.control(style.prefix());
            let suffix = self.shell.control(style.suffix());
            let content = &self.config.segment_separators.wire_left;
            write!(self.out, "{prefix}{content}{suffix}")
        } else {
            let fg = self.prev_bg.unwrap_or(Colour::White);
            let style = next_bg
                .iter()
                .fold(Style::new().fg(fg), |style, bg| style.on(*bg));
            let prefix = self.shell.control(style.prefix());
            let suffix = self.shell.control(style.suffix());
            let content = &self.config.segment_separators.solid_left;
            write!(self.out, "{prefix}{content}{suffix}")
        }
    }

    pub fn display_line(&mut self, left_segments: impl Iterator<Item = Segment>) -> io::Result<()> {
        for (i, seg) in left_segments.enumerate() {
            if i > 0 {
                self.display_left_separator(seg.style.background)?;
            }

            self.display_segment(&seg)?;
            self.prev_bg = seg.style.background;
        }

        self.display_left_closure()?;

        Ok(())
    }

    pub fn next_line(&mut self) -> io::Result<()> {
        self.prev_bg = None;
        writeln!(self.out)
    }

    pub fn finish(&mut self) -> io::Result<()> {
        write!(self.out, " ")
    }
}
