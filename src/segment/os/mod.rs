// License: MIT
// Authors:
// - Ryooooooga <eial5q265e5@gmail.com>
// - Alex Mullen <alex@xela.foo>
use crate::config::style::Style;

use super::{Context, Segment, SegmentBuilder};

#[derive(Debug, Default)]
pub struct OsSegmentBuilder {}

impl OsSegmentBuilder {
    fn build_segment(style: &Style, content: &str) -> Option<Segment> {
        Some(Segment {
            content: content.to_string(),
            style: style.to_ansi(),
        })
    }
}

impl SegmentBuilder for OsSegmentBuilder {
    #[cfg(target_os = "linux")]
    fn build(&self, ctx: &Context) -> Option<Segment> {
        use self::linux::{detect_distribution, Distribution};

        let c = &ctx.config.os;

        match detect_distribution().unwrap_or_default() {
            Distribution::Alpine => Self::build_segment(&c.alpine.style, &c.alpine.content),
            Distribution::Amazon => Self::build_segment(&c.amazon.style, &c.amazon.content),
            Distribution::Arch => Self::build_segment(&c.arch.style, &c.arch.content),
            Distribution::CentOS => Self::build_segment(&c.centos.style, &c.centos.content),
            Distribution::Debian => Self::build_segment(&c.debian.style, &c.debian.content),
            Distribution::Gentoo => Self::build_segment(&c.gentoo.style, &c.gentoo.content),
            Distribution::NixOS => Self::build_segment(&c.nix.style, &c.nix.content),
            Distribution::Raspbian => Self::build_segment(&c.raspbian.style, &c.raspbian.content),
            Distribution::Ubuntu => Self::build_segment(&c.ubuntu.style, &c.ubuntu.content),
            Distribution::Unknown => Self::build_segment(&c.linux.style, &c.linux.content),
        }
    }

    #[cfg(target_os = "macos")]
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let c = &ctx.config.os;

        Self::build_segment(&c.mac.style, &c.mac.content)
    }

    #[cfg(target_os = "windows")]
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let c = &ctx.config.os;

        Self::build_segment(&c.windows.style, &c.windows.content)
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::{fs::File, io::Read};

    #[derive(Debug, Default)]
    pub enum Distribution {
        #[default]
        Unknown,
        Alpine,
        Amazon,
        Arch,
        CentOS,
        Debian,
        Gentoo,
        NixOS,
        Raspbian,
        Ubuntu,
    }

    pub fn detect_distribution() -> Option<Distribution> {
        let mut file = File::open("/etc/os-release").ok()?;

        const HEAD_SIZE: usize = 400;
        let mut buf = [0; HEAD_SIZE];
        let size = file.read(&mut buf).ok()?;

        let head = std::str::from_utf8(&buf[..size]).ok()?;

        let mut var = "ID";
        for line in head.split('\n') {
            if let Some(id) = line.strip_prefix(format!("{}=", var).as_str()) {
                let id = id
                    .strip_prefix('"')
                    .unwrap_or(id)
                    .strip_suffix('"')
                    .unwrap_or(id);

                let distro = match id {
                    "alpine" => Some(Distribution::Alpine),
                    "amzn" => Some(Distribution::Amazon),
                    "arch" => Some(Distribution::Arch),
                    "centos" => Some(Distribution::CentOS),
                    "debian" => Some(Distribution::Debian),
                    "gentoo" => Some(Distribution::Gentoo),
                    "nixos" => Some(Distribution::NixOS),
                    "raspbian" => Some(Distribution::Raspbian),
                    "ubuntu" => Some(Distribution::Ubuntu),
                    // if ID is not configured look for ID_LIKE. this assumes ID is above ID_LIKE
                    _ => if var == "ID_LIKE" { return None } else { var = "ID_LIKE"; None },
                };
                if distro.is_some() { return distro };
            }
        }

        None
    }
}
