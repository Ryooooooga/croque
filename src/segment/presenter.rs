use super::Segment;
use crate::{config::Config, shell::Shell};
use ansi_term::{Colour, Style};
use std::fmt::{self, Write};
use std::io;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Presenter<'a> {
    config: &'a Config,
    shell: &'a Shell,
    width: usize,
}

impl<'a> Presenter<'a> {
    pub fn new(config: &'a Config, shell: &'a Shell, width: usize) -> Self {
        Self {
            config,
            shell,
            width,
        }
    }

    fn segment(&self, out: &mut String, segment: &Segment) -> usize {
        let prefix = self.shell.control(segment.style.prefix());
        let suffix = self.shell.control(segment.style.suffix());
        let content = &segment.content;
        let escaped_content = self.shell.escape(content);

        write!(out, "{prefix}{escaped_content}{suffix}").unwrap();
        content.width()
    }

    fn closure(&self, out: &mut String, color: Option<Colour>, dir: Direction) -> usize {
        let (style, content) = if let Some(color) = color {
            let style = Style::new().fg(color);
            let content = match dir {
                Direction::Left => &self.config.segment_separators.solid_left,
                Direction::Right => &self.config.segment_separators.solid_right,
            };
            (style, content)
        } else {
            let style = Style::new().fg(Colour::White);
            let content = match dir {
                Direction::Left => &self.config.segment_separators.wire_left,
                Direction::Right => &self.config.segment_separators.wire_right,
            };
            (style, content)
        };

        let prefix = self.shell.control(style.prefix());
        let suffix = self.shell.control(style.suffix());
        let escaped_content = self.shell.escape(content);

        write!(out, "{prefix}{escaped_content}{suffix}").unwrap();
        content.width()
    }

    fn separator(
        &self,
        out: &mut String,
        prev_bg: Option<Colour>,
        next_bg: Option<Colour>,
        dir: Direction,
    ) -> usize {
        let (style, content) = if next_bg == prev_bg {
            let style = next_bg
                .iter()
                .fold(Style::new().fg(Colour::White), |style, bg| style.on(*bg));
            let content = match dir {
                Direction::Left => &self.config.segment_separators.wire_left,
                Direction::Right => &self.config.segment_separators.wire_right,
            };
            (style, content)
        } else if let Direction::Left = dir {
            let fg = prev_bg.unwrap_or(Colour::White);
            let style = next_bg
                .iter()
                .fold(Style::new().fg(fg), |style, bg| style.on(*bg));
            let content = &self.config.segment_separators.solid_left;
            (style, content)
        } else {
            let fg = next_bg.unwrap_or(Colour::White);
            let style = prev_bg
                .iter()
                .fold(Style::new().fg(fg), |style, bg| style.on(*bg));
            let content = &self.config.segment_separators.solid_right;
            (style, content)
        };

        let prefix = self.shell.control(style.prefix());
        let suffix = self.shell.control(style.suffix());
        let escaped_content = self.shell.escape(content);

        write!(out, "{prefix}{escaped_content}{suffix}").unwrap();
        content.width()
    }

    fn left_contents(&self, segments: &[Segment]) -> (String, usize) {
        let mut contents = String::new();
        let mut width = 0;
        let mut prev_bg = None;

        for (i, seg) in segments.iter().enumerate() {
            if i > 0 {
                width += self.separator(
                    &mut contents,
                    prev_bg,
                    seg.style.background,
                    Direction::Left,
                );
            }

            width += self.segment(&mut contents, seg);
            prev_bg = seg.style.background;

            if i == segments.len() - 1 {
                width += self.closure(&mut contents, prev_bg, Direction::Left);
            }
        }

        (contents, width)
    }

    fn right_contents(&self, segments: &[Segment]) -> (String, usize) {
        let mut contents = String::new();
        let mut width = 0;
        let mut prev_bg = None;

        for (i, seg) in segments.iter().enumerate() {
            let next_bg = seg.style.background;
            if i == 0 {
                width += self.closure(&mut contents, next_bg, Direction::Right);
            } else {
                width += self.separator(&mut contents, prev_bg, next_bg, Direction::Right);
            }

            width += self.segment(&mut contents, seg);
            prev_bg = next_bg;
        }

        (contents, width)
    }

    pub fn display_line<W: io::Write>(
        &self,
        mut out: W,
        left: &[Segment],
        right: &[Segment],
    ) -> io::Result<()> {
        let (left_contents, left_width) = self.left_contents(left);
        let (right_contents, right_width) = self.right_contents(right);

        write!(out, "{left_contents}")?;

        if right_width > 0 && left_width + right_width + 1 < self.width {
            let right_pos = self.width - right_width - 1;
            let align_right = self.shell.control(MoveTo(right_pos));
            write!(out, "{align_right}{right_contents}")?;
        }

        Ok(())
    }

    pub fn next_line<W: io::Write>(&self, mut out: W) -> io::Result<()> {
        writeln!(out)?;
        Ok(())
    }

    pub fn finish_left<W: io::Write>(&self, mut out: W) -> io::Result<()> {
        write!(out, " ")?;
        Ok(())
    }

    pub fn display_right<W: io::Write>(&self, mut out: W, right: &[Segment]) -> io::Result<()> {
        let (right_contents, _) = self.right_contents(right);

        write!(out, "{right_contents}")?;
        Ok(())
    }
}

#[derive(Debug)]
struct MoveTo(usize);

impl fmt::Display for MoveTo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[{}G", self.0 + 1)
    }
}
