use std::fs;
use std::process::Command;
use std::str;

/// Check if the serialized email has any defects. It uses
/// Python's stdlib email to parse the serialized email and
/// then check for any defects in it.
pub fn check_defects(serialized: &str) {
    fs::write("testfile", serialized).unwrap();
    let cmd = Command::new("./checkdefects.py")
        .arg("testfile")
        .output()
        .expect("Failed to run checkdefects.py");
    assert_eq!(
        cmd.status.success(),
        true,
        "{:?}",
        str::from_utf8(&cmd.stderr).unwrap()
    );
}
