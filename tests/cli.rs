use assert_cmd::Command;

#[test]
fn dies_no_args() {
    Command::cargo_bin("sconv").unwrap()
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}
