use crate::theme::{colors, fonts, layout};
use egui::{FontId, Pos2, Rect, Sense, Ui, Vec2};

/// Activity items for the activity bar
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActivityItem {
    Explorer,
    Search,
    Git,
    Extensions,
}

impl ActivityItem {
    fn icon(&self) -> &'static str {
        match self {
            ActivityItem::Explorer => "E",
            ActivityItem::Search => "S",
            ActivityItem::Git => "G",
            ActivityItem::Extensions => "X",
        }
    }

    fn tooltip(&self) -> &'static str {
        match self {
            ActivityItem::Explorer => "Explorer (Ctrl+Shift+E)",
            ActivityItem::Search => "Search (Ctrl+Shift+F)",
            ActivityItem::Git => "Source Control (Ctrl+Shift+G)",
            ActivityItem::Extensions => "Extensions (Ctrl+Shift+X)",
        }
    }
}

/// VSCode-style activity bar widget
pub struct ActivityBar {
    active_item: ActivityItem,
    git_changes: usize,
}

impl ActivityBar {
    pub fn new(active_item: ActivityItem) -> Self {
        Self {
            active_item,
            git_changes: 0,
        }
    }

    pub fn git_changes(mut self, count: usize) -> Self {
        self.git_changes = count;
        self
    }

    pub fn show(self, ui: &mut Ui) -> ActivityBarResponse {
        let available_height = ui.available_height();
        let desired_size = Vec2::new(layout::ACTIVITY_BAR_WIDTH, available_height);
        let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::hover());

        let mut clicked_item: Option<ActivityItem> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            // Draw background
            painter.rect_filled(rect, 0.0, colors::ACTIVITY_BAR_BG);

            let items = [
                ActivityItem::Explorer,
                ActivityItem::Search,
                ActivityItem::Git,
                ActivityItem::Extensions,
            ];

            let font_id = FontId::proportional(fonts::ACTIVITY_ICON);

            for (i, item) in items.iter().enumerate() {
                let item_y = rect.top() + (i as f32 * layout::ACTIVITY_ITEM_HEIGHT);
                let item_rect = Rect::from_min_size(
                    Pos2::new(rect.left(), item_y),
                    Vec2::new(layout::ACTIVITY_BAR_WIDTH, layout::ACTIVITY_ITEM_HEIGHT),
                );

                let item_center = item_rect.center();
                let is_active = *item == self.active_item;

                // Check hover
                let item_response = ui.interact(item_rect, ui.id().with(i), Sense::click());
                let is_hovered = item_response.hovered();

                // Draw hover background
                if is_hovered && !is_active {
                    painter.rect_filled(item_rect, 0.0, colors::WIDGET_HOVERED);
                }

                // Draw active indicator (left border)
                if is_active {
                    let indicator_rect = Rect::from_min_size(
                        Pos2::new(rect.left(), item_y),
                        Vec2::new(3.0, layout::ACTIVITY_ITEM_HEIGHT),
                    );
                    painter.rect_filled(indicator_rect, 0.0, colors::ACCENT);
                }

                // Determine icon color
                let icon_color = if is_active {
                    colors::ACTIVITY_BAR_ACTIVE
                } else {
                    colors::ACTIVITY_BAR_INACTIVE
                };

                // Draw icon
                painter.text(
                    item_center,
                    egui::Align2::CENTER_CENTER,
                    item.icon(),
                    font_id.clone(),
                    icon_color,
                );

                // Draw git badge
                if *item == ActivityItem::Git && self.git_changes > 0 {
                    let badge_center = Pos2::new(item_center.x + 10.0, item_center.y - 10.0);
                    painter.circle_filled(badge_center, 8.0, colors::ACTIVITY_BAR_BADGE_BG);
                    painter.text(
                        badge_center,
                        egui::Align2::CENTER_CENTER,
                        self.git_changes.to_string(),
                        FontId::proportional(9.0),
                        colors::TEXT_PRIMARY,
                    );
                }

                // Handle click
                if item_response.clicked() {
                    clicked_item = Some(*item);
                }

                // Show tooltip on hover
                if is_hovered {
                    item_response.on_hover_text(item.tooltip());
                }
            }
        }

        ActivityBarResponse { clicked_item }
    }
}

pub struct ActivityBarResponse {
    pub clicked_item: Option<ActivityItem>,
}
