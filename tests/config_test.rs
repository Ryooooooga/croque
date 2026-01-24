mod helpers;

use crate::helpers::TestEnv;

fn run_config(env: &TestEnv) -> String {
    let (stdout, stderr) = env.run_command(&["config"]).unwrap();
    assert_eq!(stderr, "");
    stdout
}

#[test]
fn test_config() {
    let env = &TestEnv::new();
    let stdout = run_config(env);

    insta::assert_snapshot!(stdout);
}
