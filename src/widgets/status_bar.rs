use crate::state::CursorPosition;
use crate::theme::{colors, fonts, layout};
use egui::{Frame, Margin, RichText, Ui};

/// Information displayed in the status bar
#[derive(Clone)]
pub struct StatusBarInfo {
    pub cursor: CursorPosition,
    pub language: String,
    pub encoding: String,
    pub line_ending: String,
    pub total_lines: usize,
    pub total_chars: usize,
}

impl Default for StatusBarInfo {
    fn default() -> Self {
        Self {
            cursor: CursorPosition::default(),
            language: "Plain Text".to_string(),
            encoding: "UTF-8".to_string(),
            line_ending: if cfg!(windows) { "CRLF" } else { "LF" }.to_string(),
            total_lines: 0,
            total_chars: 0,
        }
    }
}

/// VSCode-style status bar widget
pub struct StatusBar {
    info: StatusBarInfo,
    file_name: Option<String>,
}

impl StatusBar {
    pub fn new(info: StatusBarInfo) -> Self {
        Self {
            info,
            file_name: None,
        }
    }

    pub fn file_name(mut self, name: Option<String>) -> Self {
        self.file_name = name;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        Frame::none()
            .fill(colors::STATUS_BAR_BG)
            .inner_margin(Margin::symmetric(layout::STATUS_BAR_ITEM_PADDING, 4.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 16.0;

                    let label_style = |text: &str| {
                        RichText::new(text)
                            .size(fonts::STATUS_BAR)
                            .color(colors::TEXT_PRIMARY)
                    };

                    // === Left side items ===

                    // File name (if available)
                    if let Some(name) = &self.file_name {
                        ui.label(label_style(&format!("📄 {}", name)));
                        ui.separator();
                    }

                    // === Right side items ===
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // App name
                        ui.label(
                            RichText::new("Rust Code Editor")
                                .size(fonts::STATUS_BAR)
                                .color(colors::TEXT_PRIMARY)
                                .strong(),
                        );

                        ui.separator();

                        // Cursor position: "Ln X, Col Y"
                        let cursor_text = self.info.cursor.display();
                        if ui
                            .selectable_label(false, label_style(&cursor_text))
                            .clicked()
                        {
                            // Could open "Go to Line" dialog
                        }

                        ui.separator();

                        // Line ending (LF/CRLF)
                        if ui
                            .selectable_label(false, label_style(&self.info.line_ending))
                            .clicked()
                        {
                            // Could show line ending selector
                        }

                        ui.separator();

                        // Encoding
                        if ui
                            .selectable_label(false, label_style(&self.info.encoding))
                            .clicked()
                        {
                            // Could show encoding selector
                        }

                        ui.separator();

                        // Language mode
                        if ui
                            .selectable_label(false, label_style(&self.info.language))
                            .clicked()
                        {
                            // Could show language selector
                        }

                        ui.separator();

                        // Line and character count
                        ui.label(label_style(&format!(
                            "Lines: {} | Chars: {}",
                            self.info.total_lines, self.info.total_chars
                        )));
                    });
                });
            });
    }
}

/// Helper to detect language from file extension
pub fn detect_language(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "rs" => "Rust",
        "js" => "JavaScript",
        "ts" => "TypeScript",
        "jsx" => "JavaScript React",
        "tsx" => "TypeScript React",
        "py" => "Python",
        "json" => "JSON",
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "md" => "Markdown",
        "html" => "HTML",
        "css" => "CSS",
        "scss" | "sass" => "SCSS",
        "java" => "Java",
        "c" => "C",
        "cpp" | "cc" | "cxx" => "C++",
        "h" | "hpp" => "C/C++ Header",
        "go" => "Go",
        "rb" => "Ruby",
        "php" => "PHP",
        "sh" | "bash" => "Shell Script",
        "sql" => "SQL",
        "xml" => "XML",
        "txt" => "Plain Text",
        _ => "Plain Text",
    }
}
