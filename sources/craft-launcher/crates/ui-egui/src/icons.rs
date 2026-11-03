//! Bundled images: each Craft App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Craft App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "photocraft" => egui::include_image!("../../../assets/app-icons/photocraft.png"),
        "vectorcraft" => egui::include_image!("../../../assets/app-icons/vectorcraft.png"),
        "filmcraft" => egui::include_image!("../../../assets/app-icons/filmcraft.png"),
        "lightcraft" => egui::include_image!("../../../assets/app-icons/lightcraft.png"),
        "pdfcraft" => egui::include_image!("../../../assets/app-icons/pdfcraft.png"),
        "effectcraft" => egui::include_image!("../../../assets/app-icons/effectcraft.png"),
        "designcraft" => egui::include_image!("../../../assets/app-icons/designcraft.png"),
        "wordcraft" => egui::include_image!("../../../assets/app-icons/wordcraft.png"),
        "deckcraft" => egui::include_image!("../../../assets/app-icons/deckcraft.png"),
        "gridcraft" => egui::include_image!("../../../assets/app-icons/gridcraft.png"),
        "cadcraft" => egui::include_image!("../../../assets/app-icons/cadcraft.png"),
        "soundcraft" => egui::include_image!("../../../assets/app-icons/soundcraft.png"),
        "figcraft" => egui::include_image!("../../../assets/app-icons/figcraft.png"),
        _ => return None,
    })
}

/// Original geometric grid for the personal modified edition.
pub fn artcraft_logo(_dark_background: bool) -> ImageSource<'static> {
    egui::include_image!("../../../assets/personal/suite.svg")
}

pub const ARTCRAFT_LOGO_ASPECT: f32 = 2.5;

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// Original suite grid icon.
pub fn artcraft_mark() -> ImageSource<'static> {
    egui::include_image!("../../../assets/personal/suite.svg")
}
