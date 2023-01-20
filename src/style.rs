use egui::{Color32, Stroke};
#[derive(Clone, Debug)]
pub struct Style {
    pub default_track_height: f32,
    pub default_track_bgcolor: Color32,
    pub default_track_stoke: Stroke,
    pub default_header_stoke: Stroke,
    pub default_header_bgcolor: Color32,
    pub margin_between_tracks: f32,
    pub default_header_width: f32,
    pub spliter_width:f32,
    pub spliter_color: Color32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            default_track_height: 40.0,
            default_track_bgcolor: Color32::BLACK,
            default_track_stoke: Stroke {
                width: 1.0,
                color: Color32::GRAY,
            },
            default_header_bgcolor: Color32::BLACK,
            default_header_stoke: Stroke {
                width: 1.0,
                color: Color32::GRAY
            },
            margin_between_tracks: 0.0,
            default_header_width: 60.0,
            spliter_width:2.0,
            spliter_color:Color32::WHITE,
        }
    }
}

impl Style {
    pub fn from_egui_style(style: &egui::Style) -> Self {
        Self {
            default_track_height: style.spacing.interact_size.y*4.0,
            default_track_bgcolor: style.visuals.window_fill,
            default_track_stoke: style.visuals.window_stroke,
            default_header_stoke: style.visuals.window_stroke,
            default_header_bgcolor: style.visuals.faint_bg_color,
            margin_between_tracks: style.spacing.item_spacing.y,
            default_header_width: style.spacing.slider_width,
            spliter_color: style.visuals.widgets.active.bg_fill,
            spliter_width:style.visuals.window_stroke.width
        }
    }
}
