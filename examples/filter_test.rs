use anyhow::Result;
use image::imageops::FilterType;
use std::path::Path;

/// Test different filter types to compare quality
pub fn test_filter_quality(input_path: &Path, output_dir: &Path) -> Result<()> {
    let img = image::open(input_path)?;
    let target_width = 400;
    let target_height = 300;

    let filters = vec![
        ("nearest", FilterType::Nearest),
        ("triangle", FilterType::Triangle),
        ("catmullrom", FilterType::CatmullRom),
        ("gaussian", FilterType::Gaussian),
        ("lanczos3", FilterType::Lanczos3),
    ];

    println!("Testing filters on image: {}", input_path.display());
    println!("Original size: {}x{}", img.width(), img.height());
    println!("Target size: {}x{}", target_width, target_height);
    println!();

    for (name, filter) in filters {
        let start = std::time::Instant::now();
        let resized = img.resize_exact(target_width, target_height, filter);
        let duration = start.elapsed();

        let output_path = output_dir.join(format!("test_{}.jpg", name));
        resized.save(&output_path)?;

        println!(
            "Filter: {} - Time: {:?} - Output: {}",
            name,
            duration,
            output_path.display()
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    let input_path = Path::new("images/237-536x354.jpg");
    let output_dir = Path::new("images/filter_test");

    std::fs::create_dir_all(output_dir)?;

    if input_path.exists() {
        test_filter_quality(input_path, output_dir)?;
    } else {
        println!("Test image not found at: {}", input_path.display());
    }

    Ok(())
}
