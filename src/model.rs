use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub file_path: String,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: DateTime<Utc>,
    pub subject: String,
    pub additions: usize,
    pub deletions: usize,
    pub files: Vec<FileChange>,
}

impl Commit {
    pub fn new(
        hash: String,
        short_hash: String,
        author_name: String,
        author_email: String,
        timestamp_secs: i64,
        subject: String,
        files: Vec<FileChange>,
    ) -> Self {
        let additions = files.iter().map(|f| f.additions).sum();
        let deletions = files.iter().map(|f| f.deletions).sum();
        let timestamp = Utc
            .timestamp_opt(timestamp_secs, 0)
            .single()
            .unwrap_or_else(Utc::now);

        Self {
            hash,
            short_hash,
            author_name,
            author_email,
            timestamp,
            subject,
            additions,
            deletions,
            files,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuthorStats {
    pub name: String,
    pub email: String,
    pub commits_count: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
}

impl AuthorStats {
    pub fn total_churn(&self) -> usize {
        self.total_additions + self.total_deletions
    }
}

#[derive(Debug, Clone)]
pub struct TimelineSnapshot {
    pub current_commit_index: usize,
    pub total_commits: usize,
    pub active_commit: Commit,
    pub cumulative_additions: usize,
    pub cumulative_deletions: usize,
    pub author_rankings: Vec<AuthorStats>,
}

pub fn calculate_author_rankings(commits_so_far: &[Commit]) -> Vec<AuthorStats> {
    let mut map: HashMap<String, AuthorStats> = HashMap::new();

    for commit in commits_so_far {
        let entry = map
            .entry(commit.author_email.to_lowercase())
            .or_insert_with(|| AuthorStats {
                name: commit.author_name.clone(),
                email: commit.author_email.clone(),
                commits_count: 0,
                total_additions: 0,
                total_deletions: 0,
            });

        entry.commits_count += 1;
        entry.total_additions += commit.additions;
        entry.total_deletions += commit.deletions;
    }

    let mut list: Vec<AuthorStats> = map.into_values().collect();
    list.sort_by(|a, b| {
        b.commits_count
            .cmp(&a.commits_count)
            .then_with(|| b.total_churn().cmp(&a.total_churn()))
    });
    list
}
