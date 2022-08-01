use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use filetime::{set_file_mtime, FileTime};
use std::fs;
#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::time::Duration;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    set_example_permissions();
    set_example_modification_time();
}

fn bcat_at_examples() -> Command {
    let mut cmd = Command::cargo_bin("bcat").unwrap();
    cmd.current_dir("tests/examples");
    cmd
}

#[test]
fn file_argument() {
    let exp_stdout = fs::read_to_string("./tests/cli_tests_expected/file_arg_stdout.txt").unwrap();
    bcat_at_examples()
        .arg("./a_file.txt")
        .assert()
        .success()
        .stdout(exp_stdout)
        .stderr("");
}

#[test]
fn args_help() {
    let exp_stdout = fs::read_to_string("./tests/cli_tests_expected/help_stdout.txt").unwrap();
    bcat_at_examples()
        .arg("--help")
        .assert()
        .success()
        .stdout(exp_stdout)
        .stderr("");
}

/**
* NB:
* Test is very fragile atm:
* - expects that test files owner/groups is the current user/group.
* - changes files/dirs modification time during execution, to allow stdout check vs expected times
* - changes files/dirs permission, which in general will not be same on all os/envs
* - substitutes size for directory, cause in general it will not be the same on all os
*
* BUT its good to have real render =)
*/
#[test]
fn dir_argument_headless() {
    let user = users::get_current_username()
        .expect("Failed to get current user name for test preparation");
    let group = users::get_current_groupname()
        .expect("Failed to get current user group for test preparation");
    let dir_size_bytes = fs::File::open("./tests/examples/a_dir_name")
        .unwrap()
        .metadata()
        .unwrap()
        .st_size();
    let dir_size_bytes = dir_size_bytes.to_string();

    let exp_stdout = fs::read_to_string("./tests/cli_tests_expected/dir_arg_stdout.txt").unwrap();

    let exp_stdout = exp_stdout.replace("<USER_PH>", user.to_string_lossy().as_ref());
    let exp_stdout = exp_stdout.replace("<GROUP_PH>", group.to_string_lossy().as_ref());
    let exp_stdout = exp_stdout.replace("<DIR_SIZE_PH>", &dir_size_bytes);

    bcat_at_examples()
        .arg("./")
        .arg("--headless")
        .assert()
        .success()
        .stdout(exp_stdout)
        .stderr("");
}

#[test]
fn no_args_err() {
    let exp_stderr = fs::read_to_string("./tests/cli_tests_expected/noargs_stderr.txt").unwrap();
    bcat_at_examples()
        .assert()
        .failure()
        .stdout("")
        .stderr(exp_stderr);
}

fn set_example_permissions() {
    fs::set_permissions(
        "./tests/examples/a_file.txt",
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    fs::set_permissions(
        "./tests/examples/b_file.txt",
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    fs::set_permissions(
        "./tests/examples/a_dir_name",
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    fs::set_permissions(
        "./tests/examples/b_dir_name",
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
}

fn set_example_modification_time() {
    let ft = FileTime::from_system_time(std::time::UNIX_EPOCH + Duration::from_secs(10));
    set_file_mtime("./tests/examples/a_file.txt", ft).unwrap();
    set_file_mtime("./tests/examples/b_file.txt", ft).unwrap();

    let ft = FileTime::from_system_time(std::time::UNIX_EPOCH + Duration::from_secs(3610));
    set_file_mtime("./tests/examples/a_dir_name", ft).unwrap();
    set_file_mtime("./tests/examples/b_dir_name", ft).unwrap();
}
