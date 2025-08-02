use image::imageops::FilterType;

fn main() {
    println!("Available filter types in the image crate:");
    
    // List all available filter types
    let filters = vec![
        ("Nearest", FilterType::Nearest),
        ("Triangle", FilterType::Triangle), 
        ("CatmullRom", FilterType::CatmullRom),
        ("Gaussian", FilterType::Gaussian),
        ("Lanczos3", FilterType::Lanczos3),
    ];
    
    for (name, _filter) in &filters {
        println!("- {}", name);
    }
    
    println!("\nFilter quality analysis:");
    println!("Nearest: Fastest, lowest quality - creates pixelated/blocky results");
    println!("Triangle (Bilinear): Fast, basic quality - smooth but can be blurry");
    println!("CatmullRom (Bicubic): Good balance of speed and quality");
    println!("Gaussian: Good quality, good for downscaling");
    println!("Lanczos3: Highest quality, slower - excellent for both up and downscaling");
}
