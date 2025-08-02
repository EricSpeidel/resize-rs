# Image Resizer (resize-rs)

A simple, fast image resizer with a graphical user interface built in Rust.

## Features

- **Multiple Format Support**: JPG, PNG, GIF, BMP, TIFF, WebP
- **Preset Sizes**: Common social media and web formats
- **Custom Dimensions**: Set your own width and height
- **Aspect Ratio**: Option to maintain or ignore aspect ratios
- **Batch Processing**: Resize multiple images at once
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Fast**: Built with Rust for optimal performance

## Development

### Quality Assurance

This project uses comprehensive quality tools:

- **Formatting**: `rustfmt` for consistent code style
- **Linting**: `clippy` with pedantic rules for code quality
- **Security**: `cargo audit` for vulnerability scanning
- **Testing**: Unit tests with `cargo test`

#### Quick Commands

```bash
# Install development tools
just install-tools

# Run all quality checks
just check-all

# Individual checks
just format      # Format code
just lint        # Run clippy
just test        # Run tests
just audit       # Security audit
```

## Presets Included

- Instagram Square (1080x1080)
- Instagram Story (1080x1920)
- Facebook Cover (820x312)
- Twitter Header (1500x500)
- YouTube Thumbnail (1280x720)
- HD 1080p (1920x1080)
- HD 720p (1280x720)
- Small Web (800x600)
- Thumbnail (150x150)

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd resize-rs
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

## Usage

1. **Select Images**: Click "Select Images" to choose one or more image files
2. **Choose Output Directory**: Click "Select Output Directory" to set where resized images will be saved
3. **Select Size**:
   - Choose from presets for common formats
   - Or use custom dimensions with optional aspect ratio maintenance
4. **Process**: Click "Start Processing" to resize all selected images

The resized images will be saved with a `_resized_WxH` suffix in the filename.

## Technical Details

### Dependencies

- **eframe/egui**: Modern, cross-platform GUI framework
- **image**: Pure Rust image processing library
- **rfd**: Cross-platform file dialogs
- **tokio**: Async runtime for non-blocking operations
- **anyhow**: Error handling
- **serde**: Serialization for settings

### Architecture

- `main.rs`: Application entry point
- `app.rs`: Main GUI application logic
- `resizer.rs`: Image processing functionality
- `presets.rs`: Predefined resize configurations

## Development

### Running in Development Mode

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
```

The optimized binary will be available in `target/release/`.

### Adding New Presets

Edit `src/presets.rs` and add new entries to the `PRESETS` array:

```rust
ResizePreset {
    name: "Custom Format",
    width: 1234,
    height: 5678,
    maintain_aspect_ratio: true,
},
```

## License

This project is open source. See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.
