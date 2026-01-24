mod helpers;

use std::process::Command;

use crate::helpers::TestEnv;

fn run_init(env: &TestEnv, shell: &str) -> String {
    let (stdout, stderr) = env.run_command(&["init", shell]).unwrap();
    assert_eq!(stderr, "");
    stdout
}

pub fn run_shell(env: &TestEnv, shell: &str, script: &str) -> String {
    let output = Command::new(shell)
        .args(&["-c", script])
        .current_dir(env.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(stderr, "");
    stdout
}

#[test]
fn test_init_bash() {
    let env = &TestEnv::new();
    let stdout = run_init(env, "bash");

    assert!(
        stdout.contains(r#"PS1="$(croque prompt"#),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(r#"croque::precmd() {"#), "stdout: {stdout}");
    assert!(
        stdout.contains(r#"PROMPT_COMMAND=croque::precmd"#),
        "stdout: {stdout}"
    );

    assert_eq!(run_shell(env, "bash", &stdout), "");
    insta::assert_snapshot!("init_bash", stdout);
}

#[test]
fn test_init_fish() {
    let env = &TestEnv::new();
    let stdout = run_init(env, "fish");

    assert!(
        stdout.contains(r#"function fish_prompt"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"croque prompt --exit-status"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"function fish_right_prompt"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"croque prompt --right --exit-status"#),
        "stdout: {stdout}"
    );

    assert_eq!(run_shell(env, "fish", &stdout), "");
    insta::assert_snapshot!("init_fish", stdout);
}

#[test]
fn test_init_zsh() {
    let env = &TestEnv::new();
    let stdout = run_init(env, "zsh");

    assert!(
        stdout.contains(r#"add-zsh-hook chpwd croque::chpwd"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"add-zsh-hook preexec croque::preexec"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"add-zsh-hook precmd croque::precmd"#),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(r#"croque::chpwd() {"#), "stdout: {stdout}");
    assert!(
        stdout.contains(r#"croque::preexec() {"#),
        "stdout: {stdout}"
    );
    assert!(stdout.contains(r#"croque::precmd() {"#), "stdout: {stdout}");
    assert!(stdout.contains(r#"croque::prompt() {"#), "stdout: {stdout}");
    assert!(
        stdout.contains(r#"croque::rprompt() {"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"croque prompt --exit-status="#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"croque prompt --right --exit-status="#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"setopt prompt_subst"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"PROMPT='$(croque::prompt)'"#),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(r#"RPROMPT='$(croque::rprompt)'"#),
        "stdout: {stdout}"
    );

    assert_eq!(run_shell(env, "zsh", &stdout), "");
    insta::assert_snapshot!("init_zsh", stdout);
}
