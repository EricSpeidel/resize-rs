use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use crate::presets::ResizePreset;
use crate::resizer::ImageResizer;

#[derive(Debug)]
enum ProcessingStatus {
    Idle,
    Processing { current: usize, total: usize },
    Completed { successful: usize, failed: usize },
    Error(String),
}

pub struct ImageResizerApp {
    selected_files: Vec<PathBuf>,
    output_directory: Option<PathBuf>,
    selected_preset: ResizePreset,
    custom_width: String,
    custom_height: String,
    maintain_aspect_ratio: bool,
    use_custom_size: bool,
    processing_status: ProcessingStatus,
    processing_receiver: Option<mpsc::Receiver<ProcessingStatus>>,
    log_messages: Vec<String>,
}

impl Default for ImageResizerApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageResizerApp {
    pub fn new() -> Self {
        Self {
            selected_files: Vec::new(),
            output_directory: None,
            selected_preset: ResizePreset::default(),
            custom_width: "800".to_string(),
            custom_height: "600".to_string(),
            maintain_aspect_ratio: true,
            use_custom_size: false,
            processing_status: ProcessingStatus::Idle,
            processing_receiver: None,
            log_messages: Vec::new(),
        }
    }

    fn add_log_message(&mut self, message: String) {
        self.log_messages.push(message);
        if self.log_messages.len() > 100 {
            self.log_messages.remove(0);
        }
    }

    fn select_files(&mut self) {
        if let Some(files) = FileDialog::new()
            .add_filter("Images", &ImageResizer::get_supported_extensions())
            .set_title("Select images to resize")
            .pick_files()
        {
            self.selected_files = files;
            self.add_log_message(format!("Selected {} files", self.selected_files.len()));
        }
    }

    fn select_output_directory(&mut self) {
        if let Some(dir) = FileDialog::new()
            .set_title("Select output directory")
            .pick_folder()
        {
            self.output_directory = Some(dir);
            self.add_log_message("Output directory selected".to_string());
        }
    }

    fn start_processing(&mut self) {
        if self.selected_files.is_empty() {
            self.add_log_message("No files selected".to_string());
            return;
        }

        let output_dir = if let Some(dir) = &self.output_directory {
            dir.clone()
        } else {
            self.add_log_message("No output directory selected".to_string());
            return;
        };

        let preset = if self.use_custom_size {
            let width = self.custom_width.parse().unwrap_or(800);
            let height = self.custom_height.parse().unwrap_or(600);
            ResizePreset {
                name: "Custom",
                width,
                height,
                maintain_aspect_ratio: self.maintain_aspect_ratio,
            }
        } else {
            self.selected_preset
        };

        let files = self.selected_files.clone();
        let (tx, rx) = mpsc::channel();
        self.processing_receiver = Some(rx);

        thread::spawn(move || {
            let progress_callback = |current: usize, total: usize| {
                let _ = tx.send(ProcessingStatus::Processing { current, total });
            };

            match ImageResizer::batch_resize(&files, &output_dir, &preset, progress_callback) {
                Ok(results) => {
                    let successful = results.iter().filter(|r| r.is_ok()).count();
                    let failed = results.len() - successful;
                    let _ = tx.send(ProcessingStatus::Completed { successful, failed });
                }
                Err(e) => {
                    let _ = tx.send(ProcessingStatus::Error(e.to_string()));
                }
            }
        });
    }

    fn update_processing_status(&mut self) {
        let mut new_status = None;
        let mut should_clear_receiver = false;
        let mut log_message = None;

        if let Some(ref receiver) = self.processing_receiver {
            while let Ok(status) = receiver.try_recv() {
                match &status {
                    ProcessingStatus::Processing { current, total } => {
                        log_message = Some(format!("Processing {} of {}", current + 1, total));
                    }
                    ProcessingStatus::Completed { successful, failed } => {
                        log_message = Some(format!(
                            "Processing completed: {successful} successful, {failed} failed"
                        ));
                        should_clear_receiver = true;
                    }
                    ProcessingStatus::Error(err) => {
                        log_message = Some(format!("Error: {err}"));
                        should_clear_receiver = true;
                    }
                    ProcessingStatus::Idle => {}
                }
                new_status = Some(status);
            }
        }

        if let Some(message) = log_message {
            self.add_log_message(message);
        }

        if should_clear_receiver {
            self.processing_receiver = None;
        }

        if let Some(status) = new_status {
            self.processing_status = status;
        }
    }
}

impl eframe::App for ImageResizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_processing_status();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Image Resizer");
            ui.separator();

            // File selection
            ui.horizontal(|ui| {
                if ui.button("Select Images").clicked() {
                    self.select_files();
                }
                ui.label(format!("Selected: {} files", self.selected_files.len()));
            });

            if !self.selected_files.is_empty() {
                ui.collapsing("Selected Files", |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for file in &self.selected_files {
                                ui.label(file.file_name().unwrap().to_string_lossy());
                            }
                        });
                });
            }

            ui.separator();

            // Output directory selection
            ui.horizontal(|ui| {
                if ui.button("Select Output Directory").clicked() {
                    self.select_output_directory();
                }
                if let Some(ref dir) = self.output_directory {
                    ui.label(format!("Output: {}", dir.display()));
                } else {
                    ui.label("No output directory selected");
                }
            });

            ui.separator();

            // Size settings
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.use_custom_size, false, "Use Preset");
                ui.radio_value(&mut self.use_custom_size, true, "Custom Size");
            });

            if !self.use_custom_size {
                ui.horizontal(|ui| {
                    ui.label("Preset:");
                    egui::ComboBox::from_label("")
                        .selected_text(self.selected_preset.name)
                        .show_ui(ui, |ui| {
                            for preset in ResizePreset::PRESETS {
                                ui.selectable_value(
                                    &mut self.selected_preset,
                                    *preset,
                                    format!("{} ({}x{})", preset.name, preset.width, preset.height),
                                );
                            }
                        });
                });

                ui.label(format!(
                    "Size: {}x{} (Aspect ratio: {})",
                    self.selected_preset.width,
                    self.selected_preset.height,
                    if self.selected_preset.maintain_aspect_ratio {
                        "maintained"
                    } else {
                        "ignored"
                    }
                ));
            } else {
                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.text_edit_singleline(&mut self.custom_width);
                    ui.label("Height:");
                    ui.text_edit_singleline(&mut self.custom_height);
                });

                ui.checkbox(&mut self.maintain_aspect_ratio, "Maintain aspect ratio");
            }

            ui.separator();

            // Processing controls and status
            let can_process = !self.selected_files.is_empty()
                && self.output_directory.is_some()
                && matches!(
                    self.processing_status,
                    ProcessingStatus::Idle | ProcessingStatus::Completed { .. }
                );

            ui.horizontal(|ui| {
                if ui
                    .add_enabled(can_process, egui::Button::new("Start Processing"))
                    .clicked()
                {
                    self.start_processing();
                }

                match &self.processing_status {
                    ProcessingStatus::Idle => {
                        ui.label("Ready");
                    }
                    ProcessingStatus::Processing { current, total } => {
                        ui.label(format!("Processing {} of {}", current + 1, total));
                        let progress = *current as f32 / *total as f32;
                        ui.add(egui::ProgressBar::new(progress).show_percentage());
                    }
                    ProcessingStatus::Completed { successful, failed } => {
                        ui.label(format!(
                            "Completed: {successful} successful, {failed} failed"
                        ));
                    }
                    ProcessingStatus::Error(err) => {
                        ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
                    }
                }
            });

            ui.separator();

            // Log area
            ui.collapsing("Log", |ui| {
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for message in &self.log_messages {
                            ui.label(message);
                        }
                    });
            });
        });

        // Request repaint if processing
        if matches!(self.processing_status, ProcessingStatus::Processing { .. }) {
            ctx.request_repaint();
        }
    }
}
