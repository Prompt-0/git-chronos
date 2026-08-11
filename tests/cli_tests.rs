use anyhow::Result;
use std::process::Command;
use tempfile::tempdir;

fn setup_test_repo() -> Result<tempfile::TempDir> {
    let dir = tempdir()?;
    let path = dir.path();

    // git init
    Command::new("git").arg("init").current_dir(path).output()?;
    Command::new("git")
        .arg("config")
        .arg("user.name")
        .arg("Test Author")
        .current_dir(path)
        .output()?;
    Command::new("git")
        .arg("config")
        .arg("user.email")
        .arg("test@example.com")
        .current_dir(path)
        .output()?;

    // Commit 1
    std::fs::write(path.join("file1.txt"), "hello world\nline 2\n")?;
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("feat: initial commit")
        .current_dir(path)
        .output()?;

    // Commit 2
    std::fs::write(path.join("file2.txt"), "second file content\n")?;
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("docs: add file2")
        .current_dir(path)
        .output()?;

    Ok(dir)
}

#[test]
fn test_cli_execution_on_repo() -> Result<()> {
    let repo_dir = setup_test_repo()?;
    let output_gif = repo_dir.path().join("output.gif");

    let cargo_bin = env!("CARGO_BIN_EXE_git-chronos");

    let status = Command::new(cargo_bin)
        .arg("--repo")
        .arg(repo_dir.path())
        .arg("--output")
        .arg(&output_gif)
        .arg("--theme")
        .arg("cyber")
        .arg("--fps")
        .arg("5")
        .arg("--title")
        .arg("Test Visualization")
        .status()?;

    assert!(status.success());
    assert!(output_gif.exists());
    assert!(std::fs::metadata(&output_gif)?.len() > 0);

    Ok(())
}

#[test]
fn test_cli_author_filter_and_max_commits() -> Result<()> {
    let repo_dir = setup_test_repo()?;
    let output_gif = repo_dir.path().join("filtered.gif");

    let cargo_bin = env!("CARGO_BIN_EXE_git-chronos");

    let status = Command::new(cargo_bin)
        .arg("--repo")
        .arg(repo_dir.path())
        .arg("--output")
        .arg(&output_gif)
        .arg("--author-filter")
        .arg("Test Author")
        .arg("--max-commits")
        .arg("1")
        .status()?;

    assert!(status.success());
    assert!(output_gif.exists());

    Ok(())
}
