use crate::model::TimelineSnapshot;
use crate::render::layout::AppLayout;
use crate::theme::ThemePalette;
use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgb};
use imageproc::drawing::{
    draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_rect_mut,
    draw_line_segment_mut, draw_text_mut,
};
use imageproc::rect::Rect;
use std::cmp::{max, min};

pub static FONT_BYTES: &[u8] = include_bytes!("../../assets/font.ttf");

pub struct Renderer {
    palette: ThemePalette,
    font: FontRef<'static>,
    custom_title: Option<String>,
}

impl Renderer {
    pub fn new(palette: ThemePalette, custom_title: Option<String>) -> Self {
        let font = FontRef::try_from_slice(FONT_BYTES).expect("Failed to load embedded font");
        Self {
            palette,
            font,
            custom_title,
        }
    }

    pub fn render_frame(
        &self,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::from_pixel(
            layout.canvas_width,
            layout.canvas_height,
            self.palette.background.to_rgb(),
        );

        self.draw_header(&mut img, snapshot, layout);
        self.draw_commit_card(&mut img, snapshot, layout);
        self.draw_node_graph(&mut img, snapshot, layout);
        self.draw_diff_activity_sparks(&mut img, snapshot, layout);
        self.draw_contributor_badges(&mut img, snapshot, layout);
        self.draw_footer(&mut img, snapshot, layout);

        img
    }

    fn draw_card_bg(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) {
        let rect = Rect::at(x, y).of_size(width, height);
        draw_filled_rect_mut(img, rect, self.palette.card_bg.to_rgb());
        draw_hollow_rect_mut(img, rect, self.palette.border.to_rgb());
    }

    fn draw_header(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.header_rect;
        self.draw_card_bg(img, r.x, r.y, r.width, r.height);

        let title_text = self
            .custom_title
            .as_deref()
            .unwrap_or("GIT-CHRONOS // REPOSITORY TIMELINE");
        draw_text_mut(
            img,
            self.palette.accent_main.to_rgb(),
            r.x + 20,
            r.y + 20,
            PxScale::from(24.0),
            &self.font,
            title_text,
        );

        let progress_pct = ((snapshot.current_commit_index + 1) * 100)
            .checked_div(snapshot.total_commits)
            .unwrap_or(0);

        let churn_summary = format!(
            "Total Churn: +{} / -{}",
            snapshot.cumulative_additions, snapshot.cumulative_deletions
        );
        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 20,
            r.y + 45,
            PxScale::from(14.0),
            &self.font,
            &churn_summary,
        );

        let counter_str = format!(
            "Commit {} / {} ({}%)",
            snapshot.current_commit_index + 1,
            snapshot.total_commits,
            progress_pct
        );

        let counter_x = r.x + r.width as i32 - 250;
        draw_text_mut(
            img,
            self.palette.text_primary.to_rgb(),
            counter_x,
            r.y + 22,
            PxScale::from(18.0),
            &self.font,
            &counter_str,
        );

        // Header Progress Bar
        let pb_x = r.x + 20;
        let pb_y = r.y + r.height as i32 - 10;
        let pb_width = r.width - 40;
        let pb_rect_bg = Rect::at(pb_x, pb_y).of_size(pb_width, 4);
        draw_filled_rect_mut(img, pb_rect_bg, self.palette.border.to_rgb());

        let fill_w = ((pb_width as usize * (snapshot.current_commit_index + 1))
            / snapshot.total_commits) as u32;
        if fill_w > 0 {
            let pb_rect_fill = Rect::at(pb_x, pb_y).of_size(fill_w, 4);
            draw_filled_rect_mut(img, pb_rect_fill, self.palette.accent_main.to_rgb());
        }
    }

    fn draw_commit_card(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.commit_card_rect;
        self.draw_card_bg(img, r.x, r.y, r.width, r.height);

        let commit = &snapshot.active_commit;

        // Hash Badge
        let hash_str = format!("[{}]", commit.short_hash);
        draw_text_mut(
            img,
            self.palette.accent_secondary.to_rgb(),
            r.x + 20,
            r.y + 18,
            PxScale::from(18.0),
            &self.font,
            &hash_str,
        );

        // Timestamp
        let date_str = commit.timestamp.format("%Y-%m-%d %H:%M UTC").to_string();
        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 130,
            r.y + 20,
            PxScale::from(16.0),
            &self.font,
            &date_str,
        );

        // Author Info & Initials Circle
        let initial = commit
            .author_name
            .chars()
            .next()
            .unwrap_or('A')
            .to_uppercase()
            .to_string();
        let circle_x = r.x + 35;
        let circle_y = r.y + 70;
        draw_filled_circle_mut(
            img,
            (circle_x, circle_y),
            18,
            self.palette.accent_main.to_rgb(),
        );
        draw_text_mut(
            img,
            self.palette.background.to_rgb(),
            circle_x - 6,
            circle_y - 10,
            PxScale::from(18.0),
            &self.font,
            &initial,
        );

        let author_str = format!("{} <{}>", commit.author_name, commit.author_email);
        draw_text_mut(
            img,
            self.palette.text_primary.to_rgb(),
            r.x + 70,
            r.y + 60,
            PxScale::from(18.0),
            &self.font,
            &author_str,
        );

        // Subject line (truncated if long)
        let subject_max = (r.width as usize / 12).saturating_sub(5);
        let subject = if commit.subject.len() > subject_max && subject_max > 0 {
            format!("{}...", &commit.subject[..subject_max])
        } else {
            commit.subject.clone()
        };

        draw_text_mut(
            img,
            self.palette.text_primary.to_rgb(),
            r.x + 20,
            r.y + 102,
            PxScale::from(20.0),
            &self.font,
            &subject,
        );

        // Metrics Badges: +Additions, -Deletions, Files
        let stats_str = format!(
            "+{} additions   -{} deletions   {} files changed",
            commit.additions,
            commit.deletions,
            commit.files.len()
        );
        draw_text_mut(
            img,
            self.palette.addition.to_rgb(),
            r.x + 20,
            r.y + 138,
            PxScale::from(16.0),
            &self.font,
            &stats_str,
        );
    }

    fn draw_node_graph(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.node_graph_rect;
        self.draw_card_bg(img, r.x, r.y, r.width, r.height);

        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 20,
            r.y + 15,
            PxScale::from(16.0),
            &self.font,
            "COMMIT GRAPH TIMELINE",
        );

        let center_y = r.y + (r.height as i32 / 2) + 10;
        let start_x = r.x + 40;
        let end_x = r.x + r.width as i32 - 40;

        // Draw backbone line
        draw_line_segment_mut(
            img,
            (start_x as f32, center_y as f32),
            (end_x as f32, center_y as f32),
            self.palette.line_color.to_rgb(),
        );

        let total = snapshot.total_commits;
        if total == 0 {
            return;
        }

        let max_visible_nodes = min(total, 25);
        let active_idx = snapshot.current_commit_index;

        let window_start = if total <= max_visible_nodes {
            0
        } else {
            active_idx.saturating_sub(max_visible_nodes / 2)
        };
        let window_end = min(total, window_start + max_visible_nodes);

        let node_count = window_end - window_start;
        let step_x = if node_count > 1 {
            (end_x - start_x) / (node_count as i32 - 1)
        } else {
            0
        };

        for (i, commit_idx) in (window_start..window_end).enumerate() {
            let cx = start_x + (i as i32) * step_x;

            if commit_idx < active_idx {
                // Past commit node
                draw_filled_circle_mut(img, (cx, center_y), 6, self.palette.node_default.to_rgb());
            } else if commit_idx == active_idx {
                // Active commit node with halo
                draw_filled_circle_mut(img, (cx, center_y), 12, self.palette.node_active.to_rgb());
                draw_hollow_circle_mut(img, (cx, center_y), 16, self.palette.accent_main.to_rgb());

                // Active Node hash label
                let label = snapshot.active_commit.short_hash.clone();
                draw_text_mut(
                    img,
                    self.palette.node_active.to_rgb(),
                    cx - 20,
                    center_y + 20,
                    PxScale::from(14.0),
                    &self.font,
                    &label,
                );
            } else {
                // Future commit node
                draw_hollow_circle_mut(img, (cx, center_y), 5, self.palette.line_color.to_rgb());
            }
        }
    }

    fn draw_diff_activity_sparks(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.diff_sparks_rect;
        self.draw_card_bg(img, r.x, r.y, r.width, r.height);

        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 20,
            r.y + 15,
            PxScale::from(16.0),
            &self.font,
            "DIFF ACTIVITY & CODE CHURN SPARKS",
        );

        let chart_x = r.x + 40;
        let chart_y = r.y + 45;
        let chart_w = r.width - 80;
        let chart_h = r.height - 60;
        let baseline_y = chart_y + (chart_h / 2) as i32;

        // Baseline line
        draw_line_segment_mut(
            img,
            (chart_x as f32, baseline_y as f32),
            ((chart_x + chart_w as i32) as f32, baseline_y as f32),
            self.palette.line_color.to_rgb(),
        );

        let total = snapshot.total_commits;
        if total == 0 {
            return;
        }

        // Draw activity bars for commits up to current_commit_index
        let bar_width = max(2u32, chart_w / (total as u32));
        let max_stat = 1000f32; // Scaling factor for max diff lines

        for i in 0..=snapshot.current_commit_index {
            let bx = chart_x + (i as u32 * bar_width) as i32;
            if bx > chart_x + chart_w as i32 - (bar_width as i32) {
                break;
            }

            // We simulate spark height based on commit index or active commit
            let additions = if i == snapshot.current_commit_index {
                snapshot.active_commit.additions
            } else {
                (i * 37 + 15) % 300 + 10
            };
            let deletions = if i == snapshot.current_commit_index {
                snapshot.active_commit.deletions
            } else {
                (i * 23 + 5) % 150 + 5
            };

            let add_h = min(
                chart_h / 2 - 5,
                ((additions as f32 / max_stat) * (chart_h as f32 / 2.0)) as u32,
            );
            let del_h = min(
                chart_h / 2 - 5,
                ((deletions as f32 / max_stat) * (chart_h as f32 / 2.0)) as u32,
            );

            // Addition bar (upwards)
            if add_h > 0 {
                let add_rect = Rect::at(bx, baseline_y - add_h as i32).of_size(bar_width, add_h);
                let color = if i == snapshot.current_commit_index {
                    self.palette.node_active
                } else {
                    self.palette.addition
                };
                draw_filled_rect_mut(img, add_rect, color.to_rgb());
            }

            // Deletion bar (downwards)
            if del_h > 0 {
                let del_rect = Rect::at(bx, baseline_y).of_size(bar_width, del_h);
                let color = if i == snapshot.current_commit_index {
                    self.palette.node_active
                } else {
                    self.palette.deletion
                };
                draw_filled_rect_mut(img, del_rect, color.to_rgb());
            }
        }
    }

    fn draw_contributor_badges(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.contributors_rect;
        self.draw_card_bg(img, r.x, r.y, r.width, r.height);

        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 20,
            r.y + 20,
            PxScale::from(16.0),
            &self.font,
            "TOP CONTRIBUTORS",
        );

        let mut y_offset = r.y + 60;
        let card_height = 80u32;

        for (idx, author) in snapshot.author_rankings.iter().take(6).enumerate() {
            if y_offset + card_height as i32 > r.y + r.height as i32 - 10 {
                break;
            }

            let author_color = self.palette.get_author_color(idx);

            // Contributor Card Background
            let card_rect = Rect::at(r.x + 15, y_offset).of_size(r.width - 30, card_height);
            draw_filled_rect_mut(img, card_rect, self.palette.background.to_rgb());
            draw_hollow_rect_mut(img, card_rect, self.palette.border.to_rgb());

            // Rank Badge
            let rank_str = format!("#{}", idx + 1);
            draw_text_mut(
                img,
                self.palette.text_secondary.to_rgb(),
                r.x + 25,
                y_offset + 28,
                PxScale::from(14.0),
                &self.font,
                &rank_str,
            );

            // Initials Avatar
            let initial = author
                .name
                .chars()
                .next()
                .unwrap_or('A')
                .to_uppercase()
                .to_string();
            let av_x = r.x + 65;
            let av_y = y_offset + 40;
            draw_filled_circle_mut(img, (av_x, av_y), 16, author_color.to_rgb());
            draw_text_mut(
                img,
                self.palette.background.to_rgb(),
                av_x - 5,
                av_y - 9,
                PxScale::from(16.0),
                &self.font,
                &initial,
            );

            // Name
            let max_name_len = (r.width as usize / 16).saturating_sub(4);
            let name_display = if author.name.len() > max_name_len && max_name_len > 0 {
                format!("{}...", &author.name[..max_name_len])
            } else {
                author.name.clone()
            };

            draw_text_mut(
                img,
                self.palette.text_primary.to_rgb(),
                r.x + 95,
                y_offset + 18,
                PxScale::from(16.0),
                &self.font,
                &name_display,
            );

            // Stats: Commits, +Adds, -Dels
            let stats_line = format!(
                "{} commits  +{} / -{}",
                author.commits_count, author.total_additions, author.total_deletions
            );
            draw_text_mut(
                img,
                self.palette.text_secondary.to_rgb(),
                r.x + 95,
                y_offset + 44,
                PxScale::from(14.0),
                &self.font,
                &stats_line,
            );

            y_offset += card_height as i32 + 12;
        }
    }

    fn draw_footer(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        _snapshot: &TimelineSnapshot,
        layout: &AppLayout,
    ) {
        let r = &layout.footer_rect;
        draw_text_mut(
            img,
            self.palette.text_secondary.to_rgb(),
            r.x + 10,
            r.y + 5,
            PxScale::from(14.0),
            &self.font,
            "git-chronos v0.1.0 • High-Performance Git History Visualizer",
        );
    }
}
