mod cli;
mod export;
mod git;
mod model;
mod render;
mod theme;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use export::Exporter;
use git::GitParser;
use render::AnimationEngine;
use std::time::Instant;
use theme::ThemePalette;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    println!("⚡ git-chronos v0.1.0");
    println!("--------------------------------------------------");
    println!("📂 Repository: {:?}", cli.repo);

    let format = cli.resolved_format();
    let author_filter = cli.author_filter.as_deref();

    let commits = GitParser::parse_repository(&cli.repo, author_filter, cli.max_commits)?;
    let total_commits = commits.len();

    let authors_count = model::calculate_author_rankings(&commits).len();
    let total_additions: usize = commits.iter().map(|c| c.additions).sum();
    let total_deletions: usize = commits.iter().map(|c| c.deletions).sum();

    println!(
        "📊 Parsed {} commits by {} authors",
        total_commits, authors_count
    );
    println!(
        "📈 Total Churn: +{} additions / -{} deletions",
        total_additions, total_deletions
    );
    println!(
        "🎨 Theme: {:?} | Canvas: {}x{} | FPS: {}",
        cli.theme, cli.width, cli.height, cli.fps
    );

    println!("🎬 Rendering timeline animation frames...");
    let palette = ThemePalette::from_mode(cli.theme);
    let frames = AnimationEngine::generate_frames(
        &commits,
        cli.width,
        cli.height,
        palette,
        cli.title.clone(),
    );

    println!("💾 Exporting to {:?} ({:?})...", cli.output, format);
    Exporter::export_animation(&frames, &cli.output, format, cli.fps)?;

    let duration = start_time.elapsed();
    let file_size = std::fs::metadata(&cli.output).map(|m| m.len()).unwrap_or(0);

    println!("--------------------------------------------------");
    println!(
        "✨ Success! Saved animation to {:?} ({:.2} MB) in {:.2}s",
        cli.output,
        file_size as f64 / 1_048_576.0,
        duration.as_secs_f64()
    );

    Ok(())
}
