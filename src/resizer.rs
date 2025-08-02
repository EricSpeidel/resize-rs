use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::{Path, PathBuf};

use crate::presets::{OutputFormat, ResizePreset};

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

        // Determine output format from preset or file extension
        let format = Self::get_output_format(&preset.output_format, input_path)?;

        // Save the resized image with appropriate quality settings
        Self::save_image_with_format(&resized_img, output_path, format)?;

        Ok(())
    }

    fn calculate_aspect_ratio_size(
        img: &DynamicImage,
        target_width: u32,
        target_height: u32,
    ) -> (u32, u32) {
        let (original_width, original_height) = img.dimensions();
        let original_aspect_ratio = f64::from(original_width) / f64::from(original_height);

        // Always set the smaller side to the target, and calculate the other to preserve aspect ratio
        if target_width <= target_height {
            // Width is the smaller side
            let width = target_width;
            let height = (f64::from(width) / original_aspect_ratio).round() as u32;
            (width, height)
        } else {
            // Height is the smaller side
            let height = target_height;
            let width = (f64::from(height) * original_aspect_ratio).round() as u32;
            (width, height)
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

    fn get_output_format(output_format: &OutputFormat, input_path: &Path) -> Result<ImageFormat> {
        match output_format {
            OutputFormat::KeepOriginal => Self::get_image_format(input_path),
            OutputFormat::Jpeg => Ok(ImageFormat::Jpeg),
            OutputFormat::Png => Ok(ImageFormat::Png),
            OutputFormat::Webp => Ok(ImageFormat::WebP),
            OutputFormat::Bmp => Ok(ImageFormat::Bmp),
            OutputFormat::Tiff => Ok(ImageFormat::Tiff),
        }
    }

    fn save_image_with_format(
        img: &DynamicImage,
        output_path: &Path,
        format: ImageFormat,
    ) -> Result<()> {
        match format {
            ImageFormat::Jpeg => {
                // Save JPEG with maximum quality (100)
                use image::codecs::jpeg::JpegEncoder;
                use std::fs::File;

                let file = File::create(output_path).with_context(|| {
                    format!("Failed to create output file: {}", output_path.display())
                })?;
                let mut encoder = JpegEncoder::new_with_quality(file, 100);
                encoder
                    .encode_image(img)
                    .with_context(|| format!("Failed to encode JPEG: {}", output_path.display()))?;
            }
            _ => {
                // For all other formats, use the standard save method
                img.save_with_format(output_path, format)
                    .with_context(|| format!("Failed to save image: {}", output_path.display()))?;
            }
        }
        Ok(())
    }

    fn get_extension_for_format(output_format: &OutputFormat, input_path: &Path) -> Result<String> {
        match output_format {
            OutputFormat::KeepOriginal => Ok(input_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()),
            OutputFormat::Jpeg => Ok("jpg".to_string()),
            OutputFormat::Png => Ok("png".to_string()),
            OutputFormat::Webp => Ok("webp".to_string()),
            OutputFormat::Bmp => Ok("bmp".to_string()),
            OutputFormat::Tiff => Ok("tiff".to_string()),
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

            // Determine output extension based on format
            let output_extension =
                Self::get_extension_for_format(&preset.output_format, input_path)?;

            let output_path = output_dir.join(format!(
                "{}_resized_{}x{}.{}",
                input_path.file_stem().unwrap().to_string_lossy(),
                preset.width,
                preset.height,
                output_extension
            ));

            let result = Self::resize_image(input_path, &output_path, preset).map(|()| output_path);

            results.push(result);
        }

        progress_callback(input_files.len(), input_files.len());
        Ok(results)
    }
}
