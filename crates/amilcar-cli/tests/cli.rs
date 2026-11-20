use std::fs;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_directory(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the Unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "amilcar-cli-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("temporary directory should be created");
    path
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_amilcar"))
        .args(args)
        .output()
        .expect("CLI process should start")
}

#[test]
fn info_supports_machine_readable_json() {
    let output = run(&["--json", "info"]);
    assert!(output.status.success());
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("output should be JSON");
    assert_eq!(value["name"], "AmilcarCore-RS");
    assert_eq!(value["execution_mode"], "local");
}

#[test]
fn text_stats_reports_utf8_counts_as_json() {
    let root = temporary_directory("text");
    let source = root.join("notes.txt");
    fs::write(&source, "Hola mundo\nAmilcarCore").unwrap();
    let output = run(&["--json", "text", "stats", source.to_str().unwrap()]);
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["words"], 3);
    assert_eq!(value["lines"], 2);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn csv_select_writes_only_requested_columns() {
    let root = temporary_directory("csv");
    let source = root.join("input.csv");
    let destination = root.join("selected.csv");
    fs::write(&source, "name,age,city\nAna,20,Lima\n").unwrap();

    let output = run(&[
        "csv",
        "select",
        source.to_str().unwrap(),
        destination.to_str().unwrap(),
        "--columns",
        "city,name",
    ]);
    assert!(output.status.success());
    let result = fs::read_to_string(&destination).unwrap();
    assert!(result.contains("city,name"));
    assert!(result.contains("Lima,Ana"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn json_format_refuses_to_overwrite_without_explicit_flag() {
    let root = temporary_directory("json");
    let source = root.join("source.json");
    let destination = root.join("destination.json");
    fs::write(&source, r#"{"ok":true}"#).unwrap();
    fs::write(&destination, "keep").unwrap();

    let output = run(&[
        "json",
        "format",
        source.to_str().unwrap(),
        destination.to_str().unwrap(),
    ]);
    assert!(!output.status.success());
    assert_eq!(fs::read_to_string(&destination).unwrap(), "keep");
    fs::remove_dir_all(root).unwrap();
}
