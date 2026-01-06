use egui::{Color32, FontFamily, FontId, Stroke, Style, TextStyle, Visuals};

// VSCode Dark+ color palette
pub mod colors {
    use egui::Color32;

    // Background colors
    pub const WINDOW_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const PANEL_BG: Color32 = Color32::from_rgb(37, 37, 38);
    pub const EDITOR_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const MENU_BAR_BG: Color32 = Color32::from_rgb(51, 51, 51);
    pub const STATUS_BAR_BG: Color32 = Color32::from_rgb(0, 122, 204);

    // Widget backgrounds
    pub const WIDGET_BG: Color32 = Color32::from_rgb(37, 37, 38);
    pub const WIDGET_INACTIVE: Color32 = Color32::from_rgb(45, 45, 45);
    pub const WIDGET_HOVERED: Color32 = Color32::from_rgb(60, 60, 60);
    pub const ACCENT: Color32 = Color32::from_rgb(0, 122, 204);

    // Tab colors
    pub const TAB_ACTIVE_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const TAB_INACTIVE_BG: Color32 = Color32::from_rgb(45, 45, 45);
    pub const TAB_MODIFIED_DOT: Color32 = Color32::WHITE;

    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::WHITE;
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(170, 170, 170);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(204, 204, 204);
    pub const TEXT_FALLBACK: Color32 = Color32::from_rgb(212, 212, 212);

    // Selection
    pub const SELECTION_BG: Color32 = Color32::from_rgb(38, 79, 120);

    // Editor gutter colors
    pub const LINE_NUMBER: Color32 = Color32::from_rgb(133, 133, 133);
    pub const LINE_NUMBER_ACTIVE: Color32 = Color32::from_rgb(199, 199, 199);
    pub const CURRENT_LINE_BG: Color32 = Color32::from_rgb(40, 40, 40);
    pub const GUTTER_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const GUTTER_BORDER: Color32 = Color32::from_rgb(50, 50, 50);

    // Indent guide colors (VSCode style)
    pub const INDENT_GUIDE: Color32 = Color32::from_rgb(64, 64, 64);
    pub const INDENT_GUIDE_ACTIVE: Color32 = Color32::from_rgb(115, 115, 115);

    // Bracket matching colors (VSCode style)
    pub const BRACKET_MATCH_BG: Color32 = Color32::from_rgba_premultiplied(0, 100, 150, 60);
    pub const BRACKET_MATCH_BORDER: Color32 = Color32::from_rgb(100, 150, 180);

    // Find/Replace colors (VSCode style)
    pub const FIND_MATCH_BG: Color32 = Color32::from_rgba_premultiplied(234, 92, 0, 70);
    pub const FIND_MATCH_CURRENT_BG: Color32 = Color32::from_rgba_premultiplied(81, 92, 106, 150);
    pub const FIND_MATCH_BORDER: Color32 = Color32::from_rgb(234, 128, 64);
    pub const FIND_PANEL_BG: Color32 = Color32::from_rgb(37, 37, 38);

    // Activity bar colors
    pub const ACTIVITY_BAR_BG: Color32 = Color32::from_rgb(51, 51, 51);
    pub const ACTIVITY_BAR_ACTIVE: Color32 = Color32::WHITE;
    pub const ACTIVITY_BAR_INACTIVE: Color32 = Color32::from_rgb(133, 133, 133);
    pub const ACTIVITY_BAR_BADGE_BG: Color32 = Color32::from_rgb(0, 122, 204);

    // Minimap colors
    pub const MINIMAP_BG: Color32 = Color32::from_rgb(30, 30, 30);
    pub const MINIMAP_VIEWPORT: Color32 = Color32::from_rgb(60, 60, 60);
    pub const MINIMAP_CODE: Color32 = Color32::from_rgb(150, 150, 150);

    // File tree colors
    pub const FILE_TREE_HOVER: Color32 = Color32::from_rgb(45, 45, 45);
    pub const FILE_TREE_SELECTED: Color32 = Color32::from_rgb(55, 55, 55);
}

// Font sizes
pub mod fonts {
    pub const SMALL: f32 = 10.0;
    pub const BODY: f32 = 13.0;
    pub const HEADING: f32 = 16.0;
    pub const STATUS_BAR: f32 = 12.0;
    pub const EXPLORER_HEADER: f32 = 11.0;
    pub const CLOSE_BUTTON: f32 = 16.0;
    pub const LINE_NUMBER: f32 = 13.0;
    pub const ACTIVITY_ICON: f32 = 24.0;
}

// Layout constants
pub mod layout {
    // Sidebar
    pub const SIDEBAR_DEFAULT_WIDTH: f32 = 250.0;
    pub const SIDEBAR_MIN_WIDTH: f32 = 150.0;
    pub const INDENT_SIZE: f32 = 16.0;

    // Gutter
    pub const GUTTER_PADDING_LEFT: f32 = 8.0;
    pub const GUTTER_PADDING_RIGHT: f32 = 12.0;

    // Activity bar
    pub const ACTIVITY_BAR_WIDTH: f32 = 50.0;
    pub const ACTIVITY_ITEM_HEIGHT: f32 = 50.0;

    // Minimap
    pub const MINIMAP_WIDTH: f32 = 100.0;
    pub const MINIMAP_LINE_HEIGHT: f32 = 2.0;
    pub const MINIMAP_CHAR_WIDTH: f32 = 1.2;

    // Tab bar
    pub const TAB_PADDING_H: f32 = 12.0;
    pub const TAB_PADDING_V: f32 = 8.0;
    pub const TAB_MODIFIED_DOT_SIZE: f32 = 8.0;

    // Status bar
    pub const STATUS_BAR_HEIGHT: f32 = 22.0;
    pub const STATUS_BAR_ITEM_PADDING: f32 = 10.0;

    // Editor
    pub const LINE_HEIGHT: f32 = 18.0;
    pub const TAB_SIZE: usize = 4;  // Number of spaces per indent level
}

pub fn create_vscode_style() -> Style {
    let mut style = Style {
        visuals: Visuals::dark(),
        ..Default::default()
    };

    style.visuals.window_fill = colors::WINDOW_BG;
    style.visuals.panel_fill = colors::PANEL_BG;
    style.visuals.faint_bg_color = colors::WIDGET_INACTIVE;
    style.visuals.extreme_bg_color = Color32::from_rgb(25, 25, 25);
    style.visuals.code_bg_color = colors::WINDOW_BG;

    style.visuals.widgets.noninteractive.bg_fill = colors::WIDGET_BG;
    style.visuals.widgets.inactive.bg_fill = colors::WIDGET_INACTIVE;
    style.visuals.widgets.hovered.bg_fill = colors::WIDGET_HOVERED;
    style.visuals.widgets.active.bg_fill = colors::ACCENT;

    style.visuals.selection.bg_fill = colors::SELECTION_BG;
    style.visuals.selection.stroke = Stroke::new(1.0, colors::ACCENT);

    // Remove widget rounding and strokes for flat VSCode look
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::ZERO;
    style.visuals.widgets.inactive.rounding = egui::Rounding::ZERO;

    // Remove window/panel shadows and rounding
    style.visuals.window_rounding = egui::Rounding::ZERO;
    style.visuals.window_shadow = egui::epaint::Shadow::NONE;
    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;

    // Reduce default spacing
    style.spacing.item_spacing = egui::Vec2::new(4.0, 2.0);
    style.spacing.window_margin = egui::Margin::same(0.0);

    // Scrollbar styling - slim like VSCode
    style.spacing.scroll = egui::style::ScrollStyle {
        bar_width: 14.0,
        handle_min_length: 20.0,
        bar_inner_margin: 2.0,
        bar_outer_margin: 0.0,
        floating: true,  // Overlay scrollbar
        ..Default::default()
    };

    style.text_styles = [
        (TextStyle::Small, FontId::new(fonts::SMALL, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(fonts::BODY, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(fonts::BODY, FontFamily::Proportional)),
        (TextStyle::Heading, FontId::new(fonts::HEADING, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(fonts::BODY, FontFamily::Monospace)),
    ]
    .into();

    style
}
