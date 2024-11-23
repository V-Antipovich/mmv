mod common;
use assert_cmd::Command;
use common::{destroy_env, setup_env, ROOT_DIRECTORY_NAME};
#[test]
fn integration_test1() {
    destroy_env();
    setup_env();
    let root = ROOT_DIRECTORY_NAME.to_string();
    let mut command = Command::cargo_bin("mmv").unwrap();
    let arguments = vec![
        root.clone() + "/path/to/some_*_filename.*",
        root.clone() + "/path/to/changed_#1_filename.#2",
    ];
    let assert1 = command.args(&arguments).assert();
    assert1.success().stdout(
        root.clone()
            + "/path/to/some_A_filename.txt -> "
            + &root
            + "/path/to/changed_A_filename.txt\n"
            + &root
            + "/path/to/some_B_filename.jpg -> "
            + &root
            + "/path/to/changed_B_filename.jpg\n"
            + &root
            + "/path/to/some__filename.gif -> "
            + &root
            + "/path/to/changed__filename.gif\n"
            + &root
            + "/path/to/some_jnskfjnes_filename.c -> "
            + &root
            + "/path/to/changed_jnskfjnes_filename.c\n"
            + "mmv: Succeded!\n",
    );
    destroy_env();
}

#[test]
fn integration_test2() {
    destroy_env();
    setup_env();
    let root = ROOT_DIRECTORY_NAME.to_string();
    let mut command = Command::cargo_bin("mmv").unwrap();
    let arguments = vec![
        root.clone() + "/path/to/some__*_filename.*",
        root.clone() + "/path/to/changed_#1_filename.#2",
    ];
    let assert1 = command.args(&arguments).assert();
    assert1.failure().code(1)
    .stdout("mmv: Files for pattern 'dehftcbt4yu3h53r5435ergieruh/path/to/some__*_filename.*' not found\n");
    destroy_env();
}

#[test]
fn integration_test3() {
    destroy_env();
    setup_env();
    let root = ROOT_DIRECTORY_NAME.to_string();
    let mut command = Command::cargo_bin("mmv").unwrap();
    let arguments = vec![
        root.clone() + "/path/to/some_*_filename.*",
        root.clone() + "/path/to/some_#1_filename.#2",
    ];
    let assert1 = command.args(&arguments).assert();
    assert1.failure().code(1).stdout(
        "mmv: Not able to replace existing file: ".to_string()
            + &root
            + "/path/to/some_A_filename.txt\n",
    );

    let mut command2 = Command::cargo_bin("mmv").unwrap();
    let arguments2 = vec![
        root.clone() + "/path/to/some_*_filename.*",
        root.clone() + "/path/to/some_#1_filename.#2",
        "-f".to_string(),
    ];
    let assert2 = command2.args(&arguments2).assert();
    assert2.success();

    destroy_env();
}
