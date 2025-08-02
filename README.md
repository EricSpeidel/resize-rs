# resize-rs

A simple GUI tool for batch image resizing, built with Rust.

## Features

- **Batch processing**: Resize multiple images at once
- **Multiple formats**: JPG, PNG, WebP, BMP, TIFF
- **Preset sizes**: Common social media and web dimensions
- **Custom dimensions**: Set your own width and height  
- **Aspect ratio control**: Maintain or ignore original proportions
- **Cross-platform**: Windows, macOS, and Linux

## Installation

### Download Release
Download the latest release for your platform from the [Releases page](https://github.com/EricSpeidel/resize-rs/releases).

### Build from Source
```bash
git clone https://github.com/EricSpeidel/resize-rs.git
cd resize-rs
cargo build --release
```

## Usage

1. Click **"Select Images"** to choose your image files
2. Click **"Select Output Directory"** to set the destination folder
3. Choose a preset size or enter custom dimensions
4. Click **"Start Processing"** to resize all images

Resized images are saved with a `_resized_WxH` suffix.

Comes with preconfigured presets for common social media and web formats, or use custom dimensions.

## License

MIT License - see [LICENSE](LICENSE) file for details.
