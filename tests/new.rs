use rember::*;
use std::process::Command;
use assert_cmd::prelude::*;
use assert_fs;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn calling_rember_without_args() {
  let mut cmd = Command::cargo_bin("rember").unwrap();
  cmd.args(&["new"]).assert().failure();
}

#[test]
fn generates_a_project() {
    let temp = assert_fs::TempDir::new().unwrap().persist_if(true);
    let project_dir = temp.child("new-application/");

    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.args(&["new", "new-application"])
        .current_dir(temp.path())
        .assert();
    project_dir.assert(predicate::path::is_dir());

    temp.close().unwrap();
}