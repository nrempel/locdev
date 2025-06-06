use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

/// Helper to create a hostie command
fn hostie_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_hostie"))
}

/// Helper to create a hostie command with a custom hosts file
fn hostie_command_with_hosts_file(hosts_file_path: &str) -> Command {
    let mut cmd = hostie_command();
    cmd.env("HOSTIE_HOSTS_FILE", hosts_file_path);
    cmd
}

/// Helper to create a test hosts file with initial content
fn create_test_hosts_file(content: &str) -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    fs::write(file.path(), content).unwrap();
    file
}

#[test]
fn test_help_flag() {
    let output = hostie_command()
        .arg("--help")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("command-line utility for managing your /etc/hosts file"));
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_version_flag() {
    let output = hostie_command()
        .arg("--version")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("hostie"));
}

#[test]
fn test_list_with_real_hosts_file() {
    // This test uses the actual hosts file (/etc/hosts on Unix, C:\Windows\System32\drivers\etc\hosts on Windows)
    // It should work without sudo/admin since we're only reading
    let output = hostie_command()
        .arg("list")
        .output()
        .expect("Failed to execute hostie");

    // On some systems (especially Windows), the hosts file might not be readable without admin privileges
    // So we'll accept either success or a permission error
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap_or_default();
        // Check if it's a permission error, which is acceptable
        assert!(
            stderr.contains("Permission denied")
                || stderr.contains("Access is denied")
                || stderr.contains("io error"),
            "Unexpected error: {}",
            stderr
        );
    }
    // If it succeeds, that's great too - just verify it runs
}

#[test]
fn test_list_empty_hosts_file() {
    let hosts_file = create_test_hosts_file("");
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .arg("list")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Should not contain any actual host entries
    assert!(!stdout.contains("127.0.0.1"));
    assert!(!stdout.contains("192.168"));
}

#[test]
fn test_list_hosts_with_entries() {
    let initial_content = "127.0.0.1 localhost\n192.168.1.1 router.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .arg("list")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("localhost"));
    assert!(stdout.contains("router.local"));
}

#[test]
fn test_add_new_entry() {
    let initial_content = "127.0.0.1 localhost\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());

    // Verify the entry was added
    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(content.contains("192.168.1.100 test.local"));
    assert!(content.contains("127.0.0.1 localhost")); // Original entry should remain
}

#[test]
fn test_add_duplicate_hostname() {
    let initial_content = "127.0.0.1 localhost\n192.168.1.100 test.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.200", "test.local"]) // Different IP, same hostname
        .output()
        .expect("Failed to execute hostie");

    // Should fail because hostname already exists
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("already exists"));
}

#[test]
fn test_remove_existing_entry() {
    let initial_content = "127.0.0.1 localhost\n192.168.1.100 test.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["remove", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());

    // Verify the entry was removed
    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(!content.contains("192.168.1.100 test.local"));
    assert!(content.contains("127.0.0.1 localhost")); // Other entries should remain
}

#[test]
fn test_remove_nonexistent_entry() {
    let initial_content = "127.0.0.1 localhost\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["remove", "192.168.1.100", "nonexistent.local"])
        .output()
        .expect("Failed to execute hostie");

    // Should fail because entry doesn't exist
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("does not exist"));
}

#[test]
fn test_remove_protected_entry() {
    let initial_content = "127.0.0.1 localhost\n192.168.1.100 test.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["remove", "127.0.0.1", "localhost"])
        .output()
        .expect("Failed to execute hostie");

    // Should fail because localhost is protected
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("protected"));
}

#[test]
fn test_handles_comments_and_empty_lines() {
    let initial_content = r#"# This is a comment
127.0.0.1 localhost

# Another comment
192.168.1.1 router.local
"#;
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .arg("list")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show entries but not comments
    assert!(stdout.contains("localhost"));
    assert!(stdout.contains("router.local"));
    assert!(!stdout.contains("# This is a comment"));
}

#[test]
fn test_add_to_empty_file() {
    let hosts_file = create_test_hosts_file("");

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());

    // Verify the entry was added
    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(content.contains("192.168.1.100 test.local"));
}

#[test]
fn test_multiple_operations() {
    let initial_content = "127.0.0.1 localhost\n";
    let hosts_file = create_test_hosts_file(initial_content);
    let hosts_path = hosts_file.path().to_str().unwrap();

    // Add an entry
    let output = hostie_command_with_hosts_file(hosts_path)
        .args(["add", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");
    assert!(output.status.success());

    // List entries
    let output = hostie_command_with_hosts_file(hosts_path)
        .arg("list")
        .output()
        .expect("Failed to execute hostie");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("localhost"));
    assert!(stdout.contains("test.local"));

    // Remove the entry
    let output = hostie_command_with_hosts_file(hosts_path)
        .args(["remove", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");
    assert!(output.status.success());

    // Verify it's gone
    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(!content.contains("192.168.1.100 test.local"));
    assert!(content.contains("127.0.0.1 localhost"));
}

// Tests for error cases without sudo (these still apply)
#[test]
fn test_add_without_sudo_fails() {
    // Skip this test if running as root (like in CI containers) or on Windows where permissions work differently
    if std::env::var("USER").unwrap_or_default() == "root" || cfg!(windows) {
        return;
    }

    // This should fail because we don't have permission to write to /etc/hosts
    let output = hostie_command()
        .args(["add", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // Should get a permission denied error
    assert!(stderr.contains("Permission denied") || stderr.contains("io error"));
}

#[test]
fn test_remove_without_sudo_fails() {
    // Skip this test if running as root (like in CI containers) or on Windows where permissions work differently
    if std::env::var("USER").unwrap_or_default() == "root" || cfg!(windows) {
        return;
    }

    // This should fail because we don't have permission to write to /etc/hosts
    let output = hostie_command()
        .args(["remove", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // Should get either a permission denied error or entry not found
    assert!(
        stderr.contains("Permission denied")
            || stderr.contains("io error")
            || stderr.contains("does not exist")
    );
}

#[test]
fn test_invalid_command() {
    let output = hostie_command()
        .arg("invalid-command")
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("error:") || stderr.contains("invalid"));
}

#[test]
fn test_add_missing_arguments() {
    let output = hostie_command()
        .args(["add", "192.168.1.100"])
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required") || stderr.contains("argument"));
}

#[test]
fn test_remove_missing_arguments() {
    let output = hostie_command()
        .args(["remove", "192.168.1.100"])
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required") || stderr.contains("argument"));
}

// NEW TESTS TO DEMONSTRATE BUGS AND SPECIFY CORRECT BEHAVIOR

#[test]
fn test_add_hostname_collision_false_positive_bug() {
    // BUG: Current implementation uses line.ends_with() which causes false positives
    let initial_content = "127.0.0.1 localhost\n192.168.1.1 mytest.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    // This should succeed because "test.local" is different from "mytest.local"
    // But current implementation will fail because "mytest.local".ends_with("test.local") is false
    // Actually, let me test the real bug case:
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.200", "host"]) // Should fail because "localhost".ends_with("host") is true
        .output()
        .expect("Failed to execute hostie");

    // Current buggy behavior: this will fail even though "host" != "localhost"
    // Correct behavior: this should succeed because they're different hostnames
    assert!(
        output.status.success(),
        "Should allow adding 'host' when 'localhost' exists"
    );

    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(content.contains("192.168.1.200 host"));
}

#[test]
fn test_add_exact_hostname_duplicate_should_fail() {
    // This test specifies the CORRECT behavior for actual duplicates
    let initial_content = "127.0.0.1 localhost\n192.168.1.100 test.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    // This should fail because "test.local" already exists (exact match)
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.200", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("already exists"));
}

#[test]
fn test_remove_exact_entry_only() {
    // BUG: Current implementation removes any line ending with hostname
    let initial_content = "127.0.0.1 localhost\n192.168.1.100 myhost\n192.168.1.200 host\n";
    let hosts_file = create_test_hosts_file(initial_content);

    // Remove "host" should only remove the exact "192.168.1.200 host" entry
    // It should NOT remove "myhost" even though "myhost".ends_with("host") is true
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["remove", "192.168.1.200", "host"])
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());

    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(
        !content.contains("192.168.1.200 host"),
        "Should remove the exact entry"
    );
    assert!(
        content.contains("192.168.1.100 myhost"),
        "Should NOT remove similar hostnames"
    );
    assert!(
        content.contains("127.0.0.1 localhost"),
        "Should preserve other entries"
    );
}

#[test]
fn test_remove_by_hostname_only_removes_matching_ip() {
    // Current implementation checks exact IP+hostname match for existence but removes by hostname only
    // This is inconsistent - let's specify the correct behavior
    let initial_content = "127.0.0.1 test.local\n192.168.1.100 test.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    // Remove specific IP+hostname combination
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["remove", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());

    let content = fs::read_to_string(hosts_file.path()).unwrap();
    assert!(
        !content.contains("192.168.1.100 test.local"),
        "Should remove the specific entry"
    );
    assert!(
        content.contains("127.0.0.1 test.local"),
        "Should preserve other IPs with same hostname"
    );
}

#[test]
fn test_file_format_preservation() {
    // Test that we don't add extra newlines or mess up formatting
    let initial_content = "127.0.0.1 localhost\n192.168.1.1 router.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    // Add an entry
    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .args(["add", "192.168.1.100", "test.local"])
        .output()
        .expect("Failed to execute hostie");
    assert!(output.status.success());

    let content = fs::read_to_string(hosts_file.path()).unwrap();

    // Should not have double newlines or extra whitespace
    assert!(
        !content.contains("\n\n"),
        "Should not create double newlines"
    );
    assert!(content.ends_with('\n'), "Should end with single newline");

    // Count lines to ensure proper formatting
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 3, "Should have exactly 3 lines");
    assert!(
        lines[2] == "192.168.1.100 test.local",
        "New entry should be properly formatted"
    );
}

#[test]
fn test_whitespace_handling_in_entries() {
    // Test that we handle various whitespace scenarios correctly
    let initial_content = "127.0.0.1\tlocalhost\n192.168.1.1  router.local\n";
    let hosts_file = create_test_hosts_file(initial_content);

    let output = hostie_command_with_hosts_file(hosts_file.path().to_str().unwrap())
        .arg("list")
        .output()
        .expect("Failed to execute hostie");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should handle both tab and multiple spaces
    assert!(stdout.contains("localhost"));
    assert!(stdout.contains("router.local"));
}
