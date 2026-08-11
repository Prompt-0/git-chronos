pub mod draw;
pub mod layout;

use crate::model::{calculate_author_rankings, Commit, TimelineSnapshot};
use crate::render::draw::Renderer;
use crate::render::layout::AppLayout;
use crate::theme::ThemePalette;
use image::{ImageBuffer, Rgb};

pub struct AnimationEngine;

impl AnimationEngine {
    pub fn generate_frames(
        commits: &[Commit],
        width: u32,
        height: u32,
        palette: ThemePalette,
        custom_title: Option<String>,
    ) -> Vec<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let layout = AppLayout::calculate(width, height);
        let renderer = Renderer::new(palette, custom_title);
        let mut frames = Vec::with_capacity(commits.len());

        let total_commits = commits.len();

        for (idx, commit) in commits.iter().enumerate() {
            let commits_so_far = &commits[0..=idx];
            let cumulative_additions = commits_so_far.iter().map(|c| c.additions).sum();
            let cumulative_deletions = commits_so_far.iter().map(|c| c.deletions).sum();
            let author_rankings = calculate_author_rankings(commits_so_far);

            let snapshot = TimelineSnapshot {
                current_commit_index: idx,
                total_commits,
                active_commit: commit.clone(),
                cumulative_additions,
                cumulative_deletions,
                author_rankings,
            };

            let frame = renderer.render_frame(&snapshot, &layout);
            frames.push(frame);
        }

        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::FileChange;
    use crate::theme::ThemePalette;

    #[test]
    fn test_animation_engine_frame_generation() {
        let commits = vec![
            Commit::new(
                "1111111111111111111111111111111111111111".to_string(),
                "1111111".to_string(),
                "Alice".to_string(),
                "alice@example.com".to_string(),
                1700000000,
                "Initial commit".to_string(),
                vec![FileChange {
                    file_path: "main.rs".to_string(),
                    additions: 10,
                    deletions: 0,
                }],
            ),
            Commit::new(
                "2222222222222222222222222222222222222222".to_string(),
                "2222222".to_string(),
                "Bob".to_string(),
                "bob@example.com".to_string(),
                1700001000,
                "Add docs".to_string(),
                vec![FileChange {
                    file_path: "README.md".to_string(),
                    additions: 20,
                    deletions: 2,
                }],
            ),
        ];

        let palette = ThemePalette::dark();
        let frames = AnimationEngine::generate_frames(&commits, 800, 600, palette, None);

        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0].width(), 800);
        assert_eq!(frames[0].height(), 600);

        // Verify image is non-empty / non-zero pixels
        let sample_pixel = frames[0].get_pixel(10, 10);
        assert_ne!(sample_pixel.0, [0, 0, 0]);
    }
}
