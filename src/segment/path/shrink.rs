use std::path::Path;

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

const PATH_SEPARATOR: &str = "/";

pub fn shrink_path(path: &Path, shrink_enabled: bool, shrink_len: usize) -> String {
    let mut reversed_path_segments = Vec::new();

    let mut path = path;
    loop {
        let first = reversed_path_segments.is_empty();

        let basename = path.file_name().unwrap_or_default().to_string_lossy();

        let should_shrink = shrink_enabled && !first;
        let basename = if should_shrink {
            shrink_file_name(&basename, shrink_len)
        } else {
            &basename
        };

        reversed_path_segments.push(basename.to_string());

        path = match path.parent() {
            Some(parent) => parent,
            None => break,
        };
    }

    let mut path_segments = reversed_path_segments;
    path_segments.reverse();

    path_segments.join(PATH_SEPARATOR)
}

#[test]
fn test_shrink_path() {
    use std::path::PathBuf;

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/ayaka/repos/repo_a"), true, 1),
        "/h/a/r/repo_a"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/ayaka/repos/repo_a"), false, 1),
        "/home/ayaka/repos/repo_a"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/ayaka/.config/croque/"), true, 2),
        "/ho/ay/.co/croque"
    );

    assert_eq!(
        &shrink_path(&PathBuf::from("/home/ayaka/今日の献立/2023 02 14"), true, 2),
        "/ho/ay/今日/2023 02 14"
    );
}
