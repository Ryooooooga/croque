use crate::config::path::PathAlias;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

fn expand_tilde<'a>(path: &str, home: Option<&'a Path>) -> Cow<'a, Path> {
    let home = match home {
        Some(home) => home,
        None => return Cow::from(PathBuf::from(path)),
    };

    if path == "~" {
        Cow::from(home)
    } else if let Some(rest) = path.strip_prefix("~/") {
        let mut result = PathBuf::from(home);
        result.push(rest);
        Cow::from(result)
    } else {
        Cow::from(PathBuf::from(path))
    }
}

#[test]
fn test_expand_tilde() {
    assert_eq!(
        expand_tilde("~", Some(&PathBuf::from("/home/ayaka"))).as_ref(),
        &PathBuf::from("/home/ayaka")
    );

    assert_eq!(
        expand_tilde("~/a", Some(&PathBuf::from("/home/ayaka"))).as_ref(),
        &PathBuf::from("/home/ayaka/a")
    );

    assert_eq!(
        expand_tilde("/~/a", Some(&PathBuf::from("/home/ayaka"))).as_ref(),
        &PathBuf::from("/~/a")
    );

    assert_eq!(expand_tilde("~/a", None).as_ref(), &PathBuf::from("~/a"));

    assert_eq!(
        expand_tilde("~a", Some(&PathBuf::from("/home/ayaka"))).as_ref(),
        &PathBuf::from("~a")
    );
}

fn shrink_file_name(file_name: &str, shrink_len: usize) -> &str {
    if shrink_len == 0 {
        return "";
    }

    let n = if file_name.starts_with('.') {
        shrink_len + 1
    } else {
        shrink_len
    };

    let end = file_name
        .char_indices()
        .map(|(idx, _)| idx)
        .nth(n)
        .unwrap_or(file_name.len());

    &file_name[..end]
}

#[test]
fn test_shrink_file_name() {
    assert_eq!(shrink_file_name("home", 1), "h");
    assert_eq!(shrink_file_name("home", 2), "ho");
    assert_eq!(shrink_file_name("home", 4), "home");
    assert_eq!(shrink_file_name("home", 5), "home");
    assert_eq!(shrink_file_name("home", 0), "");

    assert_eq!(shrink_file_name("あいう", 1), "あ");

    assert_eq!(shrink_file_name(".config", 1), ".c");
    assert_eq!(shrink_file_name(".config", 2), ".co");
    assert_eq!(shrink_file_name(".config", 7), ".config");
    assert_eq!(shrink_file_name(".config", 10), ".config");
    assert_eq!(shrink_file_name(".config", 0), "");

    assert_eq!(shrink_file_name("", 1), "");
}

struct ExpandedPathAlias<'a> {
    path: Cow<'a, Path>,
    alias: &'a str,
}

const PATH_SEPARATOR: &str = "/";

pub fn shrink_path(
    path: &Path,
    home: Option<&Path>,
    project_root: Option<&Path>,
    aliases: &[PathAlias],
    shrink_enabled: bool,
    shrink_len: usize,
) -> String {
    let aliases: Vec<ExpandedPathAlias> = aliases
        .iter()
        .map(|a| ExpandedPathAlias {
            path: expand_tilde(&a.path, home),
            alias: &a.alias,
        })
        .collect();

    let mut reversed_path_segments = Vec::new();

    let mut path = path;
    loop {
        if let Some(alias) = aliases.iter().find(|a| a.path == path) {
            reversed_path_segments.push(alias.alias.to_string());
            break;
        }

        let parent = match path.parent() {
            Some(parent) => parent,
            None => {
                #[cfg(target_os = "windows")]
                let root = path.to_string_lossy();
                #[cfg(not(target_os = "windows"))]
                let root = if reversed_path_segments.is_empty() {
                    path.to_string_lossy()
                } else {
                    Cow::from("")
                };
                reversed_path_segments.push(root.to_string());
                break;
            }
        };

        let is_first = reversed_path_segments.is_empty();
        let is_project_root = Some(path) == project_root;
        let should_shrink = shrink_enabled && !is_first && !is_project_root;

        let basename = path.file_name().unwrap_or_default().to_string_lossy();
        let shrinked_basename = if should_shrink {
            shrink_file_name(&basename, shrink_len)
        } else {
            &basename
        };

        reversed_path_segments.push(shrinked_basename.to_string());

        path = parent;
    }

    let mut path_segments = reversed_path_segments;
    path_segments.reverse();

    path_segments.join(PATH_SEPARATOR)
}

#[test]
fn test_shrink_path() {
    use std::path::PathBuf;

    let home = PathBuf::from("/home/ayaka");
    let home: Option<&Path> = Some(&home);

    let aliases = &[
        PathAlias {
            path: "~/.Trash".to_string(),
            alias: "".to_string(),
        },
        PathAlias {
            path: "~".to_string(),
            alias: "~".to_string(),
        },
    ];

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/repos/repo_a"),
            home,
            None,
            aliases,
            true,
            1
        ),
        "~/r/repo_a"
    );

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/repos/repo_a"),
            home,
            None,
            aliases,
            false,
            1
        ),
        "~/repos/repo_a"
    );

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/repos/repo_a/src"),
            home,
            Some(&PathBuf::from("/home/ayaka/repos/repo_a")),
            aliases,
            true,
            1
        ),
        "~/r/repo_a/src"
    );

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/.config/croque/"),
            home,
            None,
            aliases,
            true,
            2
        ),
        "~/.co/croque"
    );

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/今日の献立/2023 02 14"),
            home,
            None,
            aliases,
            true,
            2
        ),
        "~/今日/2023 02 14"
    );

    assert_eq!(
        &shrink_path(
            &PathBuf::from("/home/ayaka/.Trash/a"),
            home,
            None,
            aliases,
            true,
            2
        ),
        "/a"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home"), home, None, aliases, true, 1),
        "/home"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/"), home, None, aliases, true, 1),
        "/home"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/nyan"), home, None, aliases, true, 1),
        "/h/nyan"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/"), home, None, aliases, true, 1),
        "/"
    );
}
