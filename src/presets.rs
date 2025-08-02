use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResizePreset {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub maintain_aspect_ratio: bool,
}

impl ResizePreset {
    pub const PRESETS: &'static [Self] = &[
        Self {
            name: "340×570",
            width: 340,
            height: 570,
            maintain_aspect_ratio: true,
        },
        Self {
            name: "1040×570",
            width: 1040,
            height: 570,
            maintain_aspect_ratio: true,
        },
        Self {
            name: "Instagram Square",
            width: 1080,
            height: 1080,
            maintain_aspect_ratio: false,
        },
        Self {
            name: "Instagram Story",
            width: 1080,
            height: 1920,
            maintain_aspect_ratio: false,
        },
        Self {
            name: "Facebook Cover",
            width: 820,
            height: 312,
            maintain_aspect_ratio: false,
        },
        Self {
            name: "Twitter Header",
            width: 1500,
            height: 500,
            maintain_aspect_ratio: false,
        },
        Self {
            name: "YouTube Thumbnail",
            width: 1280,
            height: 720,
            maintain_aspect_ratio: false,
        },
        Self {
            name: "HD 1080p",
            width: 1920,
            height: 1080,
            maintain_aspect_ratio: true,
        },
        Self {
            name: "HD 720p",
            width: 1280,
            height: 720,
            maintain_aspect_ratio: true,
        },
        Self {
            name: "Small Web",
            width: 800,
            height: 600,
            maintain_aspect_ratio: true,
        },
        Self {
            name: "Thumbnail",
            width: 150,
            height: 150,
            maintain_aspect_ratio: true,
        },
    ];
}

impl Default for ResizePreset {
    fn default() -> Self {
        Self::PRESETS[0]
    }
}
