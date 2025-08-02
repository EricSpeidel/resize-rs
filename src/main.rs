//! # Resize RS
//!
//! A simple GUI application for batch image resizing with multiple format support.
//! Built with eframe/egui for a modern, cross-platform user interface.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;
mod presets;
mod resizer;

use app::ImageResizerApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "Image Resizer",
        options,
        Box::new(|_cc| Ok(Box::new(ImageResizerApp::new()))),
    )
}
