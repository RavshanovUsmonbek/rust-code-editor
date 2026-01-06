use crate::theme::{colors, fonts, layout};
use egui::{Frame, Margin, Pos2, RichText, Sense, Stroke, Ui, Vec2};

/// Represents a single tab
#[derive(Clone)]
pub struct Tab {
    pub name: String,
    pub icon: String,
    pub is_modified: bool,
}

impl Tab {
    pub fn new(name: impl Into<String>, icon: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            icon: icon.into(),
            is_modified: false,
        }
    }

    pub fn modified(mut self, is_modified: bool) -> Self {
        self.is_modified = is_modified;
        self
    }
}

/// Response from TabBar widget
pub struct TabBarResponse {
    pub activated: Option<usize>,
    pub closed: Option<usize>,
}

/// Enhanced tab bar with modified indicators
pub struct TabBar {
    tabs: Vec<Tab>,
    active_index: usize,
}

impl TabBar {
    pub fn new(tabs: Vec<Tab>, active_index: usize) -> Self {
        Self { tabs, active_index }
    }

    pub fn show(self, ui: &mut Ui) -> TabBarResponse {
        let mut response = TabBarResponse {
            activated: None,
            closed: None,
        };

        Frame::none()
            .fill(colors::PANEL_BG)
            .inner_margin(Margin::symmetric(0.0, 0.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;

                    for (i, tab) in self.tabs.iter().enumerate() {
                        let is_active = i == self.active_index;
                        let tab_response = self.render_tab(ui, tab, is_active, i);

                        if tab_response.activated {
                            response.activated = Some(i);
                        }
                        if tab_response.closed {
                            response.closed = Some(i);
                        }
                    }
                });
            });

        response
    }

    fn render_tab(
        &self,
        ui: &mut Ui,
        tab: &Tab,
        is_active: bool,
        _index: usize,
    ) -> SingleTabResponse {
        let mut activated = false;
        let mut closed = false;

        let bg_color = if is_active {
            colors::TAB_ACTIVE_BG
        } else {
            colors::TAB_INACTIVE_BG
        };

        Frame::none()
            .fill(bg_color)
            .inner_margin(Margin::symmetric(
                layout::TAB_PADDING_H,
                layout::TAB_PADDING_V,
            ))
            .show(ui, |ui| {
                // Draw top border for active tab
                if is_active {
                    let rect = ui.min_rect();
                    ui.painter().line_segment(
                        [
                            Pos2::new(rect.left(), rect.top() - layout::TAB_PADDING_V),
                            Pos2::new(rect.right(), rect.top() - layout::TAB_PADDING_V),
                        ],
                        Stroke::new(2.0, colors::ACCENT),
                    );
                }

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 0.0);

                    // Modified indicator (white dot)
                    if tab.is_modified {
                        let (dot_rect, _) = ui.allocate_exact_size(
                            Vec2::splat(layout::TAB_MODIFIED_DOT_SIZE),
                            Sense::hover(),
                        );
                        ui.painter().circle_filled(
                            dot_rect.center(),
                            layout::TAB_MODIFIED_DOT_SIZE / 2.0 - 1.0,
                            colors::TAB_MODIFIED_DOT,
                        );
                    }

                    // Icon and file name
                    let text_color = if is_active {
                        colors::TEXT_PRIMARY
                    } else {
                        colors::TEXT_SECONDARY
                    };

                    let label_text = format!("{} {}", tab.icon, tab.name);
                    let label = RichText::new(&label_text)
                        .size(fonts::BODY)
                        .color(text_color);

                    let label_response = ui.selectable_label(false, label);
                    if label_response.clicked() {
                        activated = true;
                    }

                    // Close button
                    let (close_rect, close_response) =
                        ui.allocate_exact_size(Vec2::splat(fonts::CLOSE_BUTTON), Sense::click());

                    // Draw close button
                    let close_hovered = close_response.hovered();
                    let close_color = if close_hovered {
                        colors::TEXT_PRIMARY
                    } else {
                        colors::TEXT_SECONDARY
                    };

                    if close_hovered {
                        ui.painter()
                            .rect_filled(close_rect, 2.0, colors::WIDGET_HOVERED);
                    }

                    ui.painter().text(
                        close_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "×",
                        egui::FontId::proportional(fonts::CLOSE_BUTTON),
                        close_color,
                    );

                    if close_response.clicked() {
                        closed = true;
                    }
                });
            });

        SingleTabResponse { activated, closed }
    }
}

struct SingleTabResponse {
    activated: bool,
    closed: bool,
}
