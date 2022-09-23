use assert_cmd::Command;
use oranda;

#[test]
fn it_works() {
    let mut cmd = Command::cargo_bin("oranda").unwrap();
    cmd.assert().success();
}

#[test]
fn it_prints_the_right_msg() {
    let mut cmd = Command::cargo_bin("oranda").unwrap();
    let msg = oranda::say_hello().to_owned() + "\n";
    cmd.assert().success().stdout(msg);
}
