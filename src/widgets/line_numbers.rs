use crate::theme::{colors, fonts, layout};
use egui::{FontId, Pos2, Rect, Response, Sense, Ui, Vec2};

/// Custom line numbers gutter widget that renders VSCode-style line numbers
pub struct LineNumbersGutter {
    total_lines: usize,
    current_line: usize,
    scroll_offset_y: f32,
    line_height: f32,
    visible_height: f32,
}

impl LineNumbersGutter {
    pub fn new(total_lines: usize) -> Self {
        Self {
            total_lines: total_lines.max(1),
            current_line: 1,
            scroll_offset_y: 0.0,
            line_height: layout::LINE_HEIGHT,
            visible_height: 500.0,
        }
    }

    pub fn current_line(mut self, line: usize) -> Self {
        self.current_line = line.max(1);
        self
    }

    pub fn scroll_offset(mut self, offset: f32) -> Self {
        self.scroll_offset_y = offset;
        self
    }

    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = height;
        self
    }

    pub fn visible_height(mut self, height: f32) -> Self {
        self.visible_height = height;
        self
    }

    /// Calculate the width needed for line numbers based on digit count
    fn calculate_width(&self, ui: &Ui) -> f32 {
        let max_digits = self.total_lines.to_string().len().max(3);
        let font_id = FontId::monospace(fonts::LINE_NUMBER);
        let digit_width = ui.fonts(|f| f.glyph_width(&font_id, '0'));

        (max_digits as f32 * digit_width)
            + layout::GUTTER_PADDING_LEFT
            + layout::GUTTER_PADDING_RIGHT
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let gutter_width = self.calculate_width(ui);
        let desired_size = Vec2::new(gutter_width, self.visible_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let font_id = FontId::monospace(fonts::LINE_NUMBER);

            // Draw gutter background
            painter.rect_filled(rect, 0.0, colors::GUTTER_BG);

            // Calculate visible line range
            let first_visible = (self.scroll_offset_y / self.line_height).floor() as usize;
            let visible_count = (self.visible_height / self.line_height).ceil() as usize + 2;
            let last_visible = (first_visible + visible_count).min(self.total_lines);

            // Draw each visible line number
            for line_num in (first_visible + 1)..=(last_visible) {
                if line_num > self.total_lines {
                    break;
                }

                // Calculate Y position for this line
                let line_top = ((line_num - 1) as f32 * self.line_height) - self.scroll_offset_y;

                // Skip if outside visible area
                if line_top < -self.line_height || line_top > self.visible_height {
                    continue;
                }

                let is_current = line_num == self.current_line;

                // Draw current line highlight background
                if is_current {
                    let highlight_rect = Rect::from_min_size(
                        Pos2::new(rect.left(), rect.top() + line_top),
                        Vec2::new(gutter_width, self.line_height),
                    );
                    painter.rect_filled(highlight_rect, 0.0, colors::CURRENT_LINE_BG);
                }

                // Determine text color
                let text_color = if is_current {
                    colors::LINE_NUMBER_ACTIVE
                } else {
                    colors::LINE_NUMBER
                };

                // Draw line number (right-aligned)
                let text_pos = Pos2::new(
                    rect.right() - layout::GUTTER_PADDING_RIGHT,
                    rect.top() + line_top + (self.line_height / 2.0),
                );

                painter.text(
                    text_pos,
                    egui::Align2::RIGHT_CENTER,
                    line_num.to_string(),
                    font_id.clone(),
                    text_color,
                );
            }

            // Draw right border separator (subtle line)
            painter.line_segment(
                [
                    Pos2::new(rect.right() - 0.5, rect.top()),
                    Pos2::new(rect.right() - 0.5, rect.bottom()),
                ],
                egui::Stroke::new(1.0, colors::GUTTER_BORDER),
            );
        }

        response
    }
}
