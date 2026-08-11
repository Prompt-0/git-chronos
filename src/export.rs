use crate::cli::OutputFormat;
use anyhow::{anyhow, Context, Result};
use gif::{Encoder, Frame as GifFrame, Repeat};
use image::{ImageBuffer, Rgb};
use std::fs::File;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

pub struct Exporter;

impl Exporter {
    pub fn export_animation(
        frames: &[ImageBuffer<Rgb<u8>, Vec<u8>>],
        output_path: &Path,
        format: OutputFormat,
        fps: u32,
    ) -> Result<()> {
        if frames.is_empty() {
            return Err(anyhow!("Cannot export empty frame sequence"));
        }

        match format {
            OutputFormat::Gif => Self::export_gif(frames, output_path, fps),
            OutputFormat::Mp4 => Self::export_mp4(frames, output_path, fps),
        }
    }

    pub fn export_gif(
        frames: &[ImageBuffer<Rgb<u8>, Vec<u8>>],
        output_path: &Path,
        fps: u32,
    ) -> Result<()> {
        let width = frames[0].width() as u16;
        let height = frames[0].height() as u16;

        let file = File::create(output_path)
            .with_context(|| format!("Failed to create GIF file at {:?}", output_path))?;

        let mut encoder = Encoder::new(file, width, height, &[])
            .with_context(|| "Failed to initialize GIF encoder")?;

        encoder
            .set_repeat(Repeat::Infinite)
            .with_context(|| "Failed to set GIF repeat mode")?;

        // delay in hundredths of a second
        let delay_cs = (100 / fps.max(1)) as u16;

        for (idx, frame_buf) in frames.iter().enumerate() {
            let raw_pixels = frame_buf.clone().into_raw();
            let mut gif_frame = GifFrame::from_rgb_speed(width, height, &raw_pixels, 10);
            gif_frame.delay = delay_cs;

            encoder
                .write_frame(&gif_frame)
                .with_context(|| format!("Failed to encode GIF frame #{}", idx))?;
        }

        Ok(())
    }

    pub fn export_mp4(
        frames: &[ImageBuffer<Rgb<u8>, Vec<u8>>],
        output_path: &Path,
        fps: u32,
    ) -> Result<()> {
        let ffmpeg_check = Command::new("ffmpeg").arg("-version").output();
        if ffmpeg_check.is_err() || !ffmpeg_check.as_ref().unwrap().status.success() {
            return Err(anyhow!(
                "MP4 export requires 'ffmpeg' utility installed on system PATH. Please install ffmpeg or use GIF format (-f gif)."
            ));
        }

        let dir = tempdir().with_context(|| "Failed to create temp directory for MP4 frames")?;

        for (idx, frame) in frames.iter().enumerate() {
            let frame_path = dir.path().join(format!("frame_{:05}.png", idx));
            frame
                .save(&frame_path)
                .with_context(|| format!("Failed to save frame png {:?}", frame_path))?;
        }

        let input_pattern = dir.path().join("frame_%05d.png");

        let status = Command::new("ffmpeg")
            .arg("-y")
            .arg("-framerate")
            .arg(fps.to_string())
            .arg("-i")
            .arg(input_pattern)
            .arg("-c:v")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg(output_path)
            .status()
            .with_context(|| "Failed to execute ffmpeg command")?;

        if !status.success() {
            return Err(anyhow!("ffmpeg failed to encode MP4 video file"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_gif() {
        let mut img = ImageBuffer::new(100, 100);
        for pixel in img.pixels_mut() {
            *pixel = Rgb([50, 100, 150]);
        }
        let frames = vec![img.clone(), img];

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let result = Exporter::export_gif(&frames, path, 10);
        assert!(result.is_ok());
        assert!(path.metadata().unwrap().len() > 0);
    }
}
