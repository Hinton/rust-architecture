use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn discover_fixtures() -> Vec<String> {
    let fixtures_dir = get_fixtures_dir();
    let mut fixtures: Vec<String> = fs::read_dir(fixtures_dir)
        .expect("Failed to read fixtures directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().join("expected.md").exists())
        .filter_map(|entry| entry.file_name().to_str().map(String::from))
        .collect();

    fixtures.sort();
    fixtures
}

fn run_test_for_fixture(fixture_name: &str) {
    let fixture_path = get_fixtures_dir().join(fixture_name);
    let expected = fs::read_to_string(fixture_path.join("expected.md"))
        .unwrap_or_else(|_| panic!("Failed to read expected.md for fixture: {}", fixture_name));

    let temp_dir = std::env::temp_dir().join(format!("rust-arch-test-{}", fixture_name));
    fs::create_dir_all(&temp_dir).ok();
    let output_path = temp_dir.join("ARCHITECTURE.md");

    let pattern = fixture_path.join("**/README.md");

    // Check if this fixture has a config file
    let config_path = fixture_path.join("architecture.toml");
    let has_config = config_path.exists();

    let mut args = vec![
        "run".to_string(),
        "--".to_string(),
        "generate".to_string(),
        pattern.display().to_string(),
        output_path.display().to_string(),
    ];

    if has_config {
        args.push("--config".to_string());
        args.push(config_path.display().to_string());
    }

    let output = Command::new("cargo")
        .args(&args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Generate command failed for fixture '{}': {}",
        fixture_name,
        String::from_utf8_lossy(&output.stderr)
    );

    let actual = fs::read_to_string(&output_path)
        .unwrap_or_else(|_| panic!("Failed to read output file for fixture: {}", fixture_name));

    assert_eq!(
        actual.trim(),
        expected.trim(),
        "Fixture '{}' output doesn't match expected",
        fixture_name
    );

    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_all_fixtures() {
    let fixtures = discover_fixtures();
    assert!(!fixtures.is_empty(), "No fixtures found in tests/fixtures/");

    for fixture in fixtures {
        run_test_for_fixture(&fixture);
    }
}
