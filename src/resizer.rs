use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::{Path, PathBuf};

use crate::presets::ResizePreset;

pub struct ImageResizer;

impl ImageResizer {
    pub fn resize_image(
        input_path: &Path,
        output_path: &Path,
        preset: &ResizePreset,
    ) -> Result<()> {
        // Load the image
        let img = image::open(input_path)
            .with_context(|| format!("Failed to open image: {}", input_path.display()))?;

        // Calculate new dimensions
        let (new_width, new_height) = if preset.maintain_aspect_ratio {
            Self::calculate_aspect_ratio_size(&img, preset.width, preset.height)
        } else {
            (preset.width, preset.height)
        };

        // Resize the image using highest quality filter
        // Lanczos3 is the best choice for quality - excellent for both upscaling and downscaling
        let resized_img = if preset.maintain_aspect_ratio {
            img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        } else {
            // Use resize_exact for non-aspect-ratio preserving resize
            img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3)
        };

        // Determine output format from file extension
        let format = Self::get_image_format(output_path)?;

        // Save the resized image
        resized_img
            .save_with_format(output_path, format)
            .with_context(|| format!("Failed to save image: {}", output_path.display()))?;

        Ok(())
    }

    fn calculate_aspect_ratio_size(
        img: &DynamicImage,
        target_width: u32,
        target_height: u32,
    ) -> (u32, u32) {
        let (original_width, original_height) = img.dimensions();
        let original_aspect_ratio = f64::from(original_width) / f64::from(original_height);

        // Calculate dimensions based on both target width and height
        // Use ceiling to prevent rounding down to values below target dimensions
        let height_from_width = (f64::from(target_width) / original_aspect_ratio).ceil() as u32;
        let width_from_height = (f64::from(target_height) * original_aspect_ratio).ceil() as u32;

        // Determine which dimension to prioritize to ensure the smaller side matches exactly
        if width_from_height <= target_width {
            // Height constraint is more restrictive, use exact target height
            (width_from_height, target_height)
        } else {
            // Width constraint is more restrictive, use exact target width
            (target_width, height_from_width)
        }
    }

    fn get_image_format(path: &Path) -> Result<ImageFormat> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(str::to_lowercase)
            .context("No file extension found")?;

        match extension.as_str() {
            "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
            "png" => Ok(ImageFormat::Png),
            "gif" => Ok(ImageFormat::Gif),
            "bmp" => Ok(ImageFormat::Bmp),
            "tiff" | "tif" => Ok(ImageFormat::Tiff),
            "webp" => Ok(ImageFormat::WebP),
            _ => anyhow::bail!("Unsupported image format: {}", extension),
        }
    }

    pub fn get_supported_extensions() -> Vec<&'static str> {
        vec!["jpg", "jpeg", "png", "gif", "bmp", "tiff", "tif", "webp"]
    }

    pub fn batch_resize(
        input_files: &[PathBuf],
        output_dir: &Path,
        preset: &ResizePreset,
        progress_callback: impl Fn(usize, usize),
    ) -> Result<Vec<Result<PathBuf>>> {
        let mut results = Vec::new();

        for (index, input_path) in input_files.iter().enumerate() {
            progress_callback(index, input_files.len());

            let _file_name = input_path.file_name().context("Invalid file name")?;

            let output_path = output_dir.join(format!(
                "{}_resized_{}x{}.{}",
                input_path.file_stem().unwrap().to_string_lossy(),
                preset.width,
                preset.height,
                input_path.extension().unwrap().to_string_lossy()
            ));

            let result = Self::resize_image(input_path, &output_path, preset).map(|()| output_path);

            results.push(result);
        }

        progress_callback(input_files.len(), input_files.len());
        Ok(results)
    }
}
