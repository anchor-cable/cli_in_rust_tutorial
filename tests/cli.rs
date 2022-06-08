use std::{error::Error, process::Command};
use assert_cmd::prelude::*;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

#[test]
fn given_blank_pattern() -> Result<(), Box<dyn Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(""));

    Ok(())
}


#[test]
fn file_doesnt_exit() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("cli_in_rust_tutorial")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains("could not read file"));

    Ok(())
}