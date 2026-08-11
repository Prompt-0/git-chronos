use crate::model::{Commit, FileChange};
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::Path;
use std::process::Command;

pub struct GitParser;

impl GitParser {
    pub fn parse_repository(
        repo_path: &Path,
        author_filter: Option<&str>,
        max_commits: Option<usize>,
    ) -> Result<Vec<Commit>> {
        if !repo_path.exists() {
            return Err(anyhow!("Repository path does not exist: {:?}", repo_path));
        }

        let output = Command::new("git")
            .arg("-C")
            .arg(repo_path)
            .arg("log")
            .arg("--reverse")
            .arg("--pretty=format:---COMMIT---%n%H%n%h%n%an%n%ae%n%at%n%s")
            .arg("--numstat")
            .output()
            .with_context(|| format!("Failed to execute 'git' command in {:?}", repo_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Git log command failed: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut commits = parse_git_log_output(&stdout)?;

        if let Some(filter) = author_filter {
            let filter_lc = filter.to_lowercase();
            let regex = Regex::new(filter).ok();
            commits.retain(|c| {
                if let Some(re) = &regex {
                    re.is_match(&c.author_name) || re.is_match(&c.author_email)
                } else {
                    c.author_name.to_lowercase().contains(&filter_lc)
                        || c.author_email.to_lowercase().contains(&filter_lc)
                }
            });
        }

        if let Some(max) = max_commits {
            if commits.len() > max {
                commits.truncate(max);
            }
        }

        if commits.is_empty() {
            return Err(anyhow!("No commits found matching specified parameters"));
        }

        Ok(commits)
    }
}

pub fn parse_git_log_output(raw_output: &str) -> Result<Vec<Commit>> {
    let mut commits = Vec::new();
    let sections = raw_output.split("---COMMIT---");

    for section in sections {
        let trimmed = section.trim();
        if trimmed.is_empty() {
            continue;
        }

        let lines: Vec<&str> = trimmed.lines().collect();
        if lines.len() < 6 {
            continue;
        }

        let hash = lines[0].trim().to_string();
        let short_hash = lines[1].trim().to_string();
        let author_name = lines[2].trim().to_string();
        let author_email = lines[3].trim().to_string();
        let timestamp_secs: i64 = lines[4].trim().parse().unwrap_or(0);
        let subject = lines[5].trim().to_string();

        let mut files = Vec::new();

        for line in &lines[6..] {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let additions = parts[0].parse::<usize>().unwrap_or(0);
                let deletions = parts[1].parse::<usize>().unwrap_or(0);
                let file_path = parts[2..].join(" ");

                files.push(FileChange {
                    file_path,
                    additions,
                    deletions,
                });
            }
        }

        commits.push(Commit::new(
            hash,
            short_hash,
            author_name,
            author_email,
            timestamp_secs,
            subject,
            files,
        ));
    }

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_git_log_output() {
        let raw = r#"
---COMMIT---
1111111111111111111111111111111111111111
1111111
Alice Smith
alice@example.com
1700000000
feat: initial commit
10	0	src/main.rs
5	2	Cargo.toml
---COMMIT---
2222222222222222222222222222222222222222
2222222
Bob Jones
bob@example.com
1700003600
fix: resolve bug
3	8	src/main.rs
"#;

        let commits = parse_git_log_output(raw).unwrap();
        assert_eq!(commits.len(), 2);

        let c1 = &commits[0];
        assert_eq!(c1.hash, "1111111111111111111111111111111111111111");
        assert_eq!(c1.short_hash, "1111111");
        assert_eq!(c1.author_name, "Alice Smith");
        assert_eq!(c1.author_email, "alice@example.com");
        assert_eq!(c1.subject, "feat: initial commit");
        assert_eq!(c1.additions, 15);
        assert_eq!(c1.deletions, 2);
        assert_eq!(c1.files.len(), 2);

        let c2 = &commits[1];
        assert_eq!(c2.author_name, "Bob Jones");
        assert_eq!(c2.additions, 3);
        assert_eq!(c2.deletions, 8);
    }
}
