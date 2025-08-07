use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_framework_version() {
    let mut cmd = Command::cargo_bin("aei-cli").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("AEI Framework version"));
}
