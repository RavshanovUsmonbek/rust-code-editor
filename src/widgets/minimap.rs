use egui::{Pos2, Rect, Sense, Stroke, Ui, Vec2};
use crate::theme::{colors, layout};

/// Minimap widget showing a condensed code overview
pub struct Minimap<'a> {
    text: &'a str,
    total_lines: usize,
    visible_lines: (usize, usize),
    current_line: usize,
}

impl<'a> Minimap<'a> {
    pub fn new(text: &'a str, total_lines: usize) -> Self {
        Self {
            text,
            total_lines: total_lines.max(1),
            visible_lines: (1, 50),
            current_line: 1,
        }
    }

    pub fn visible_lines(mut self, range: (usize, usize)) -> Self {
        self.visible_lines = range;
        self
    }

    pub fn current_line(mut self, line: usize) -> Self {
        self.current_line = line;
        self
    }

    pub fn show(self, ui: &mut Ui) -> MinimapResponse {
        let available_height = ui.available_height();
        let desired_size = Vec2::new(layout::MINIMAP_WIDTH, available_height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        let mut clicked_line: Option<usize> = None;

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            // Draw background
            painter.rect_filled(rect, 0.0, colors::MINIMAP_BG);

            // Calculate scale to fit all lines
            let scale = if self.total_lines > 0 {
                (rect.height() / (self.total_lines as f32 * layout::MINIMAP_LINE_HEIGHT)).min(1.0)
            } else {
                1.0
            };

            let line_height = layout::MINIMAP_LINE_HEIGHT * scale;

            // Draw viewport indicator (visible area)
            let (start_line, end_line) = self.visible_lines;
            let viewport_top = rect.top() + ((start_line.saturating_sub(1)) as f32 * line_height);
            let viewport_height = ((end_line - start_line + 1) as f32 * line_height).max(10.0);

            let viewport_rect = Rect::from_min_size(
                Pos2::new(rect.left(), viewport_top),
                Vec2::new(layout::MINIMAP_WIDTH, viewport_height),
            );
            painter.rect_filled(viewport_rect, 2.0, colors::MINIMAP_VIEWPORT);

            // Draw current line indicator
            let current_y = rect.top() + ((self.current_line.saturating_sub(1)) as f32 * line_height);
            let current_line_rect = Rect::from_min_size(
                Pos2::new(rect.left(), current_y),
                Vec2::new(layout::MINIMAP_WIDTH, line_height.max(2.0)),
            );
            painter.rect_filled(current_line_rect, 0.0, colors::CURRENT_LINE_BG);

            // Draw condensed code representation
            for (line_idx, line) in self.text.lines().enumerate() {
                let y = rect.top() + (line_idx as f32 * line_height);

                if y > rect.bottom() {
                    break;
                }

                // Calculate indent and content length
                let indent = line.chars().take_while(|c| c.is_whitespace()).count();
                let trimmed = line.trim();
                let content_len = trimmed.len().min(80);

                if content_len > 0 {
                    let x_start = rect.left() + 4.0 + (indent as f32 * layout::MINIMAP_CHAR_WIDTH * 0.5);
                    let x_end = x_start + (content_len as f32 * layout::MINIMAP_CHAR_WIDTH);

                    painter.line_segment(
                        [
                            Pos2::new(x_start, y + line_height * 0.5),
                            Pos2::new(x_end.min(rect.right() - 4.0), y + line_height * 0.5),
                        ],
                        Stroke::new(line_height * 0.6, colors::MINIMAP_CODE),
                    );
                }
            }

            // Handle click to navigate
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let relative_y = pos.y - rect.top();
                    let clicked = (relative_y / line_height) as usize + 1;
                    clicked_line = Some(clicked.min(self.total_lines));
                }
            }
        }

        MinimapResponse { clicked_line }
    }
}

pub struct MinimapResponse {
    pub clicked_line: Option<usize>,
}
