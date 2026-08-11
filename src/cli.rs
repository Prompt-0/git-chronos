use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Gif,
    Mp4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ThemeMode {
    Dark,
    Light,
    Cyber,
    Dracula,
    Solarized,
    Neon,
}

#[derive(Parser, Debug)]
#[command(
    name = "git-chronos",
    author = "Antigravity Team",
    version = "0.1.0",
    about = "Generate production-quality animated timeline visualizations of git commit history"
)]
pub struct Cli {
    /// Path to target git repository
    #[arg(short, long, default_value = ".")]
    pub repo: PathBuf,

    /// Output file path (e.g. chronos.gif, chronos.mp4)
    #[arg(short, long, default_value = "chronos.gif")]
    pub output: PathBuf,

    /// Output animation format (gif or mp4)
    #[arg(short, long, value_enum)]
    pub format: Option<OutputFormat>,

    /// Animation frame rate (frames per second)
    #[arg(long, default_value_t = 10)]
    pub fps: u32,

    /// Visual color theme
    #[arg(short, long, value_enum, default_value_t = ThemeMode::Dark)]
    pub theme: ThemeMode,

    /// Filter commits by author (name or email substring/pattern)
    #[arg(long)]
    pub author_filter: Option<String>,

    /// Maximum number of commits to process
    #[arg(short = 'n', long)]
    pub max_commits: Option<usize>,

    /// Canvas width in pixels
    #[arg(long, default_value_t = 1280)]
    pub width: u32,

    /// Canvas height in pixels
    #[arg(long, default_value_t = 720)]
    pub height: u32,

    /// Custom title for visualization header
    #[arg(long)]
    pub title: Option<String>,
}

impl Cli {
    pub fn resolved_format(&self) -> OutputFormat {
        if let Some(fmt) = self.format {
            return fmt;
        }
        if let Some(ext) = self.output.extension() {
            if ext.to_string_lossy().to_lowercase() == "mp4" {
                return OutputFormat::Mp4;
            }
        }
        OutputFormat::Gif
    }
}
