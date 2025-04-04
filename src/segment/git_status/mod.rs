use crate::{
    config::git_status::{BranchAlias, GitStatusIcons, RemoteConfig},
    info::git::{Head, RemoteStatus, UpstreamStatus, WorkingTreeStatus},
};
use aho_corasick::AhoCorasick;
use std::borrow::Cow;
use std::fmt::Write;

use super::{Context, Segment, SegmentBuilder};

#[derive(Debug)]
pub struct GitStatusSegmentBuilder {
    replacer: AhoCorasick,
}

impl GitStatusSegmentBuilder {
    pub fn new() -> Self {
        let replacer = AhoCorasick::new([
            "{{.remote}}",
            "{{.head}}",
            "{{.working_tree}}",
            "{{.upstream}}",
        ])
        .unwrap();
        Self { replacer }
    }

    fn shorten_hash(hash: &str, max_length: usize) -> &str {
        if hash.len() > max_length {
            &hash[..max_length]
        } else {
            hash
        }
    }

    fn build_remote_status(
        remotes: &[RemoteStatus],
        remote_configs: &[RemoteConfig],
    ) -> Option<String> {
        if remotes.is_empty() {
            return None;
        }

        let mut result = String::new();

        for remote in remotes {
            let matched = remote_configs
                .iter()
                .find(|c| remote.url.contains(&c.pattern));

            if let Some(matched) = matched {
                result += &matched.icon;
            }
        }

        result.push(' ');

        Some(result)
    }

    fn build_head_status<'a>(
        head: &'a Head,
        icons: &'a GitStatusIcons,
        display_master: bool,
        branch_aliases: &[BranchAlias],
        commit_hash_length: usize,
    ) -> Cow<'a, str> {
        match head {
            Head::Branch(branch) if !display_master && (branch == "master" || branch == "main") => {
                let icon = &icons.branch;
                Cow::from(icon)
            }
            Head::Branch(branch) => {
                let branch = branch_aliases
                    .iter()
                    .find(|ba| branch.starts_with(&ba.prefix))
                    .map(|ba| Cow::from(format!("{}{}", ba.alias, &branch[ba.prefix.len()..])))
                    .unwrap_or(Cow::from(branch));
                let icon = &icons.branch;
                if icon.is_empty() {
                    branch
                } else {
                    Cow::from(format!("{icon} {branch}"))
                }
            }
            Head::Tag(tag) => {
                let icon = &icons.tag;
                if icon.is_empty() {
                    Cow::from(tag)
                } else {
                    Cow::from(format!("{icon} {tag}"))
                }
            }
            Head::Commit(hash) => {
                let icon = &icons.commit;
                let short_hash = Self::shorten_hash(hash, commit_hash_length);
                if icon.is_empty() {
                    Cow::from(short_hash)
                } else {
                    Cow::from(format!("{icon} {short_hash}"))
                }
            }
        }
    }

    fn build_working_tree_status(
        working_tree: &WorkingTreeStatus,
        icons: &GitStatusIcons,
    ) -> String {
        let mut status = String::new();
        status.reserve(16);

        if working_tree.has_new() {
            let _ = write!(status, "{}", icons.added);
        }
        if working_tree.has_deleted() {
            let _ = write!(status, "{}", icons.deleted);
        }
        if working_tree.has_modified() {
            let _ = write!(status, "{}", icons.modified);
        }
        if working_tree.has_renamed() {
            let _ = write!(status, "{}", icons.renamed);
        }
        if working_tree.has_conflict() {
            let _ = write!(status, "{}", icons.conflicted);
        }

        if !status.is_empty() {
            format!(" {status}")
        } else {
            status
        }
    }

    fn build_upstream_status(upstream: &UpstreamStatus, icons: &GitStatusIcons) -> Option<String> {
        let behind_icon = &icons.behind;
        let ahead_icon = &icons.ahead;
        match (upstream.behind, upstream.ahead) {
            (0, 0) => None,
            (behind, 0) => Some(format!(" {behind_icon}{behind}")),
            (0, ahead) => Some(format!(" {ahead_icon}{ahead}")),
            (behind, ahead) => Some(format!(" {behind_icon}{behind}{ahead_icon}{ahead}")),
        }
    }
}

impl Default for GitStatusSegmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentBuilder for GitStatusSegmentBuilder {
    fn build(&self, ctx: &Context) -> Option<Segment> {
        let config = &ctx.config.git_status;
        let git_info = ctx.git_info?;

        let remote = Self::build_remote_status(&git_info.remotes, &config.remotes);
        let remote = remote.as_deref().unwrap_or_default();

        let head = Self::build_head_status(
            &git_info.head,
            &config.icons,
            config.display_master,
            &config.branch_aliases,
            config.commit_hash_length,
        );

        let working_tree = Self::build_working_tree_status(&git_info.working_tree, &config.icons);

        let upstream = git_info
            .upstream
            .as_ref()
            .and_then(|upstream| Self::build_upstream_status(upstream, &config.icons));

        let content = self.replacer.replace_all(
            &config.content,
            &[
                remote,
                head.as_ref(),
                &working_tree,
                upstream.as_deref().unwrap_or_default(),
            ],
        );

        let style = if git_info.working_tree.has_conflict() {
            &config.conflicted.style
        } else if git_info.working_tree.has_unstaged_changes() {
            &config.unstaged.style
        } else if git_info.working_tree.has_staged_changes() {
            &config.staged.style
        } else {
            &config.clean.style
        };
        let style = style.to_ansi();

        Some(Segment { content, style })
    }
}
