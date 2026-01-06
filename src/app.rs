use crate::file_icons;
use crate::fs_tree::FileNode;
use crate::state::{CursorPosition, EditorTabState};
use crate::theme::{colors, create_vscode_style, fonts, layout};
use crate::widgets::{
    status_bar::detect_language, ActivityBar, ActivityItem, LineNumbersGutter, Minimap, StatusBar,
    StatusBarInfo, Tab, TabBar,
};
use egui::{
    Color32, FontId, Frame, Margin, Pos2, Rect, RichText, ScrollArea, TextEdit, TextStyle, Vec2,
};
use rfd::FileDialog;
use ropey::Rope;
use std::path::PathBuf;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

const SYNTAX_THEME: &str = "base16-ocean.dark";

/// Bracket pairs for matching
const BRACKET_PAIRS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

/// Find the matching bracket position for a given cursor position
fn find_matching_bracket(text: &str, cursor_offset: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = text.chars().collect();

    if cursor_offset >= chars.len() {
        return None;
    }

    // Check character at cursor and before cursor
    let positions_to_check = if cursor_offset > 0 {
        vec![cursor_offset, cursor_offset - 1]
    } else {
        vec![cursor_offset]
    };

    for pos in positions_to_check {
        if pos >= chars.len() {
            continue;
        }

        let ch = chars[pos];

        // Check if it's an opening bracket
        for &(open, close) in BRACKET_PAIRS {
            if ch == open {
                // Search forward for closing bracket
                let mut depth = 1;
                for (i, &c) in chars.iter().enumerate().skip(pos + 1) {
                    if c == open {
                        depth += 1;
                    } else if c == close {
                        depth -= 1;
                        if depth == 0 {
                            return Some((pos, i));
                        }
                    }
                }
            } else if ch == close {
                // Search backward for opening bracket
                let mut depth = 1;
                for i in (0..pos).rev() {
                    if chars[i] == close {
                        depth += 1;
                    } else if chars[i] == open {
                        depth -= 1;
                        if depth == 0 {
                            return Some((i, pos));
                        }
                    }
                }
            }
        }
    }

    None
}

/// Convert character offset to (line, column) for rendering
fn offset_to_line_col(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;

    for (i, ch) in text.chars().enumerate() {
        if i == offset {
            return (line, col);
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (line, col)
}

pub struct OpenFile {
    pub path: PathBuf,
    pub buffer: Rope,
    pub original_content: String,
    pub state: EditorTabState,
}

impl OpenFile {
    fn new(path: PathBuf, content: String) -> Self {
        Self {
            path,
            buffer: Rope::from_str(&content),
            original_content: content,
            state: EditorTabState::default(),
        }
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    fn extension(&self) -> &str {
        self.path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("txt")
    }

    fn is_modified(&self) -> bool {
        self.state.is_modified
    }
}

/// Find/Replace panel state
#[derive(Default)]
struct FindReplaceState {
    is_open: bool,
    show_replace: bool,
    search_text: String,
    replace_text: String,
    case_sensitive: bool,
    current_match: usize,
    matches: Vec<(usize, usize)>, // (start_offset, end_offset)
}

pub struct EditorApp {
    workspace: Option<PathBuf>,
    tree: Vec<FileNode>,
    open_files: Vec<OpenFile>,
    active_tab: usize,
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    active_activity: ActivityItem,
    show_minimap: bool,
    editor_scroll_offset: Vec2,
    find_replace: FindReplaceState,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            workspace: None,
            tree: vec![],
            open_files: vec![],
            active_tab: 0,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            active_activity: ActivityItem::Explorer,
            show_minimap: true,
            editor_scroll_offset: Vec2::ZERO,
            find_replace: FindReplaceState::default(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_style(create_vscode_style());

        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);

        self.render_menu_bar(ctx);
        self.render_activity_bar(ctx);
        self.render_sidebar(ctx);
        self.render_editor(ctx);
        self.render_status_bar(ctx);

        // Render find/replace panel on top if open
        if self.find_replace.is_open {
            self.render_find_replace_panel(ctx);
        }
    }
}

impl EditorApp {
    // === Keyboard Shortcuts ===

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        use egui::Key;

        ctx.input(|i| {
            // Ctrl+F - Open Find
            if i.modifiers.ctrl && i.key_pressed(Key::F) {
                self.find_replace.is_open = true;
                self.find_replace.show_replace = false;
            }

            // Ctrl+H - Open Find and Replace
            if i.modifiers.ctrl && i.key_pressed(Key::H) {
                self.find_replace.is_open = true;
                self.find_replace.show_replace = true;
            }

            // Escape - Close Find panel
            if i.key_pressed(Key::Escape) && self.find_replace.is_open {
                self.find_replace.is_open = false;
            }

            // Ctrl+S - Save
            if i.modifiers.ctrl && i.key_pressed(Key::S) {
                self.save_current_file();
            }
        });
    }

    // === Find/Replace Panel ===

    fn render_find_replace_panel(&mut self, ctx: &egui::Context) {
        egui::Area::new(egui::Id::new("find_replace_panel"))
            .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-20.0, 50.0))
            .show(ctx, |ui| {
                Frame::none()
                    .fill(colors::FIND_PANEL_BG)
                    .inner_margin(Margin::same(8.0))
                    .rounding(4.0)
                    .shadow(egui::epaint::Shadow {
                        extrusion: 8.0,
                        color: Color32::from_black_alpha(100),
                    })
                    .show(ui, |ui| {
                        ui.set_min_width(320.0);

                        // Find row
                        ui.horizontal(|ui| {
                            ui.label("Find:");
                            let find_response = ui.add(
                                TextEdit::singleline(&mut self.find_replace.search_text)
                                    .desired_width(200.0)
                                    .hint_text("Search..."),
                            );

                            // Auto-search when text changes
                            if find_response.changed() {
                                self.perform_search();
                            }

                            // Request focus on first open
                            if !self.find_replace.search_text.is_empty()
                                || find_response.gained_focus()
                            {
                                find_response.request_focus();
                            }

                            // Navigation buttons
                            if ui
                                .button("▲")
                                .on_hover_text("Previous (Shift+Enter)")
                                .clicked()
                            {
                                self.find_previous();
                            }
                            if ui.button("▼").on_hover_text("Next (Enter)").clicked() {
                                self.find_next();
                            }

                            // Match count
                            let match_count = self.find_replace.matches.len();
                            if match_count > 0 {
                                let current = self.find_replace.current_match + 1;
                                ui.label(format!("{}/{}", current, match_count));
                            } else if !self.find_replace.search_text.is_empty() {
                                ui.label("No results");
                            }

                            // Close button
                            if ui.button("✕").clicked() {
                                self.find_replace.is_open = false;
                            }
                        });

                        // Replace row (if enabled)
                        if self.find_replace.show_replace {
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                ui.label("Replace:");
                                ui.add(
                                    TextEdit::singleline(&mut self.find_replace.replace_text)
                                        .desired_width(200.0)
                                        .hint_text("Replace with..."),
                                );

                                if ui
                                    .button("Replace")
                                    .on_hover_text("Replace current")
                                    .clicked()
                                {
                                    self.replace_current();
                                }
                                if ui.button("All").on_hover_text("Replace all").clicked() {
                                    self.replace_all();
                                }
                            });
                        }

                        // Options row
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.find_replace.case_sensitive, "Match case");
                            ui.checkbox(&mut self.find_replace.show_replace, "Replace");
                        });
                    });
            });
    }

    fn perform_search(&mut self) {
        self.find_replace.matches.clear();
        self.find_replace.current_match = 0;

        if self.find_replace.search_text.is_empty() {
            return;
        }

        if let Some(file) = self.open_files.get(self.active_tab) {
            let text = file.buffer.to_string();
            let search = &self.find_replace.search_text;

            let (text_to_search, search_pattern) = if self.find_replace.case_sensitive {
                (text.clone(), search.clone())
            } else {
                (text.to_lowercase(), search.to_lowercase())
            };

            let search_len = search.chars().count();
            let mut start = 0;

            while let Some(pos) = text_to_search[start..].find(&search_pattern) {
                let abs_pos = start + pos;
                // Convert byte position to char position
                let char_start = text[..abs_pos].chars().count();
                let char_end = char_start + search_len;
                self.find_replace.matches.push((char_start, char_end));
                start = abs_pos + search.len();
            }
        }
    }

    fn find_next(&mut self) {
        if !self.find_replace.matches.is_empty() {
            self.find_replace.current_match =
                (self.find_replace.current_match + 1) % self.find_replace.matches.len();
        }
    }

    fn find_previous(&mut self) {
        if !self.find_replace.matches.is_empty() {
            if self.find_replace.current_match == 0 {
                self.find_replace.current_match = self.find_replace.matches.len() - 1;
            } else {
                self.find_replace.current_match -= 1;
            }
        }
    }

    fn replace_current(&mut self) {
        if self.find_replace.matches.is_empty() {
            return;
        }

        if let Some(file) = self.open_files.get_mut(self.active_tab) {
            let (start, end) = self.find_replace.matches[self.find_replace.current_match];
            let mut text = file.buffer.to_string();
            let chars: Vec<char> = text.chars().collect();

            // Convert char positions to byte positions
            let byte_start: usize = chars[..start].iter().collect::<String>().len();
            let byte_end: usize = chars[..end].iter().collect::<String>().len();

            text.replace_range(byte_start..byte_end, &self.find_replace.replace_text);
            file.buffer = Rope::from_str(&text);
            file.state.is_modified = true;

            // Re-search to update matches
            self.perform_search();
        }
    }

    fn replace_all(&mut self) {
        if self.find_replace.matches.is_empty() {
            return;
        }

        if let Some(file) = self.open_files.get_mut(self.active_tab) {
            let text = file.buffer.to_string();
            let search = &self.find_replace.search_text;
            let replace = &self.find_replace.replace_text;

            let new_text = if self.find_replace.case_sensitive {
                text.replace(search, replace)
            } else {
                // Case-insensitive replace
                let mut result = text.clone();
                let lower_text = text.to_lowercase();
                let lower_search = search.to_lowercase();
                let mut offset: i64 = 0;

                for (pos, _) in lower_text.match_indices(&lower_search) {
                    let adjusted_pos = (pos as i64 + offset) as usize;
                    let end_pos = adjusted_pos + search.len();
                    result.replace_range(adjusted_pos..end_pos, replace);
                    offset += replace.len() as i64 - search.len() as i64;
                }
                result
            };

            file.buffer = Rope::from_str(&new_text);
            file.state.is_modified = true;

            // Re-search to update matches
            self.perform_search();
        }
    }

    // === Menu Bar ===

    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                Frame::none()
                    .fill(colors::MENU_BAR_BG)
                    .inner_margin(Margin::symmetric(8.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.spacing_mut().button_padding = Vec2::new(8.0, 4.0);

                    self.file_menu(ui);
                    self.edit_menu(ui);
                    self.view_menu(ui);
                });
            });
    }

    fn file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            ui.style_mut().spacing.item_spacing.y = 4.0;

            if ui.button("📁 Open Folder...").clicked() {
                self.open_folder();
                ui.close_menu();
            }
            if ui.button("📄 Open File...").clicked() {
                self.open_file_dialog();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("💾 Save").clicked() {
                self.save_current_file();
                ui.close_menu();
            }
        });
    }

    fn edit_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |ui| {
            ui.style_mut().spacing.item_spacing.y = 4.0;

            if ui.button("🔍 Find                    Ctrl+F").clicked() {
                self.find_replace.is_open = true;
                self.find_replace.show_replace = false;
                ui.close_menu();
            }
            if ui.button("🔄 Find and Replace   Ctrl+H").clicked() {
                self.find_replace.is_open = true;
                self.find_replace.show_replace = true;
                ui.close_menu();
            }
        });
    }

    fn view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("View", |ui| {
            if ui
                .checkbox(&mut self.show_minimap, "Show Minimap")
                .clicked()
            {
                ui.close_menu();
            }
        });
    }

    // === Activity Bar ===

    fn render_activity_bar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("activity_bar")
            .resizable(false)
            .exact_width(layout::ACTIVITY_BAR_WIDTH)
            .frame(Frame::none().fill(colors::ACTIVITY_BAR_BG))
            .show(ctx, |ui| {
                let response = ActivityBar::new(self.active_activity)
                    .git_changes(0)
                    .show(ui);

                if let Some(item) = response.clicked_item {
                    self.active_activity = item;
                }
            });
    }

    // === Sidebar ===

    fn render_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("explorer")
            .resizable(true)
            .default_width(layout::SIDEBAR_DEFAULT_WIDTH)
            .min_width(layout::SIDEBAR_MIN_WIDTH)
            .frame(
                Frame::none()
                    .fill(colors::PANEL_BG)
                    .inner_margin(Margin::same(0.0)),
            )
            .show(ctx, |ui| {
                self.render_explorer_header(ui);
                ui.separator();
                self.render_file_tree(ui);
            });
    }

    fn render_explorer_header(&self, ui: &mut egui::Ui) {
        Frame::none()
            .inner_margin(Margin::symmetric(12.0, 8.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("EXPLORER")
                        .size(fonts::EXPLORER_HEADER)
                        .color(colors::TEXT_MUTED)
                        .strong(),
                );
            });
    }

    fn render_file_tree(&mut self, ui: &mut egui::Ui) {
        let mut file_to_open: Option<PathBuf> = None;
        let active_path = self.open_files.get(self.active_tab).map(|f| f.path.clone());

        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.y = 0.0;

                for node in &self.tree {
                    Self::render_file_node(ui, node, &mut file_to_open, active_path.as_ref(), 0);
                }
                ui.add_space(8.0);
            });

        if let Some(path) = file_to_open {
            self.open_file(path);
        }
    }

    fn render_file_node(
        ui: &mut egui::Ui,
        node: &FileNode,
        file_to_open: &mut Option<PathBuf>,
        active_path: Option<&PathBuf>,
        depth: usize,
    ) {
        let indent = depth as f32 * layout::INDENT_SIZE;
        let name = node.name();
        let item_height = layout::LINE_HEIGHT + 2.0;

        if node.is_dir {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                let header = format!("{} {}", file_icons::FOLDER_ICON, name);
                egui::CollapsingHeader::new(RichText::new(header).size(fonts::BODY))
                    .default_open(depth == 0)
                    .show(ui, |ui| {
                        for child in &node.children {
                            Self::render_file_node(ui, child, file_to_open, active_path, depth + 1);
                        }
                    });
            });
        } else {
            let is_selected = active_path == Some(&node.path);
            let available_width = ui.available_width();

            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(available_width, item_height),
                egui::Sense::click(),
            );

            // Draw background for hover/selection
            let bg_color = if is_selected {
                colors::FILE_TREE_SELECTED
            } else if response.hovered() {
                colors::FILE_TREE_HOVER
            } else {
                Color32::TRANSPARENT
            };

            if bg_color != Color32::TRANSPARENT {
                ui.painter().rect_filled(rect, 0.0, bg_color);
            }

            // Draw icon and text
            let icon = file_icons::get_icon(&name);
            let text_pos = Pos2::new(
                rect.left() + indent + layout::INDENT_SIZE + 4.0,
                rect.center().y,
            );
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                format!("{} {}", icon, name),
                FontId::proportional(fonts::BODY),
                colors::TEXT_PRIMARY,
            );

            if response.clicked() {
                *file_to_open = Some(node.path.clone());
            }
        }
    }

    // === Editor ===

    fn render_editor(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(colors::EDITOR_BG)
                    .inner_margin(Margin::same(0.0)),
            )
            .show(ctx, |ui| {
                self.render_tab_bar(ui);
                ui.separator();
                self.render_editor_content(ui);
            });
    }

    fn render_tab_bar(&mut self, ui: &mut egui::Ui) {
        let tabs: Vec<Tab> = self
            .open_files
            .iter()
            .map(|f| Tab::new(f.name(), file_icons::get_icon(&f.name())).modified(f.is_modified()))
            .collect();

        if tabs.is_empty() {
            return;
        }

        Frame::none()
            .fill(colors::PANEL_BG)
            .inner_margin(Margin::symmetric(0.0, 4.0))
            .show(ui, |ui| {
                let response = TabBar::new(tabs, self.active_tab).show(ui);

                if let Some(idx) = response.activated {
                    self.active_tab = idx;
                }
                if let Some(idx) = response.closed {
                    self.close_tab(idx);
                }
            });
    }

    fn render_editor_content(&mut self, ui: &mut egui::Ui) {
        if self.open_files.is_empty() {
            self.render_welcome_screen(ui);
            return;
        }

        let active_idx = self.active_tab;
        let line_height = layout::LINE_HEIGHT;
        let available_height = ui.available_height();
        let show_minimap = self.show_minimap;
        let scroll_offset_y = self.editor_scroll_offset.y;

        // Get file info for line numbers and minimap
        let (total_lines, text_content, visible_lines, current_line) = {
            let file = &self.open_files[active_idx];
            (
                file.buffer.len_lines(),
                file.buffer.to_string(),
                file.state.visible_lines,
                file.state.cursor.line,
            )
        };

        let mut minimap_clicked_line: Option<usize> = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;

            // === Line Numbers Gutter ===
            LineNumbersGutter::new(total_lines)
                .current_line(current_line)
                .scroll_offset(scroll_offset_y)
                .line_height(line_height)
                .visible_height(available_height)
                .show(ui);

            // === Main Editor Area ===
            // Editor fills remaining space between gutter and minimap
            let minimap_width = if show_minimap {
                layout::MINIMAP_WIDTH
            } else {
                0.0
            };
            let editor_width = ui.available_width() - minimap_width;

            ui.vertical(|ui| {
                ui.set_width(editor_width);
                ui.set_height(available_height);
                self.render_text_editor(ui, line_height);
            });

            // === Minimap ===
            if show_minimap {
                let minimap_response = Minimap::new(&text_content, total_lines)
                    .visible_lines(visible_lines)
                    .current_line(current_line)
                    .show(ui);

                minimap_clicked_line = minimap_response.clicked_line;
            }
        });

        // Handle minimap click
        if let Some(clicked_line) = minimap_clicked_line {
            let target_y = (clicked_line.saturating_sub(1)) as f32 * line_height;
            self.editor_scroll_offset.y = target_y;
        }
    }

    fn render_text_editor(&mut self, ui: &mut egui::Ui, line_height: f32) {
        let active_idx = self.active_tab;
        let file = &mut self.open_files[active_idx];
        let mut text = file.buffer.to_string();
        let original = file.original_content.clone();
        let current_line = file.state.cursor.line;
        let prev_char_count = text.chars().count();

        // Get syntax highlighting info
        let ext = file.extension().to_string();
        let syntax = self
            .syntax_set
            .find_syntax_by_extension(&ext)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
            .clone();
        let theme = self.theme_set.themes[SYNTAX_THEME].clone();
        let syntax_set = self.syntax_set.clone();

        let mut layouter = |ui: &egui::Ui, text: &str, wrap_width: f32| {
            let mut job = egui::text::LayoutJob::default();
            job.wrap.max_width = wrap_width;

            let mut highlighter = HighlightLines::new(&syntax, &theme);

            for line in text.lines() {
                if let Ok(ranges) = highlighter.highlight_line(line, &syntax_set) {
                    for (style, segment) in ranges {
                        job.append(
                            segment,
                            0.0,
                            egui::TextFormat {
                                font_id: FontId::monospace(fonts::BODY),
                                color: Color32::from_rgb(
                                    style.foreground.r,
                                    style.foreground.g,
                                    style.foreground.b,
                                ),
                                line_height: Some(line_height),
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    job.append(
                        line,
                        0.0,
                        egui::TextFormat {
                            font_id: FontId::monospace(fonts::BODY),
                            color: colors::TEXT_FALLBACK,
                            line_height: Some(line_height),
                            ..Default::default()
                        },
                    );
                }
                job.append(
                    "\n",
                    0.0,
                    egui::TextFormat {
                        line_height: Some(line_height),
                        ..Default::default()
                    },
                );
            }

            ui.fonts(|f| f.layout_job(job))
        };

        // ScrollArea fills available space directly - no Frame wrapper
        let scroll_area = ScrollArea::both()
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible);

        // Calculate character width for indent guides
        let char_width = ui.fonts(|f| f.glyph_width(&FontId::monospace(fonts::BODY), ' '));

        // Pre-calculate indent levels for each line
        let indent_levels: Vec<usize> = text
            .lines()
            .map(|line| {
                let spaces = line.chars().take_while(|c| *c == ' ').count();
                let tabs = line.chars().take_while(|c| *c == '\t').count();
                (spaces + tabs * layout::TAB_SIZE) / layout::TAB_SIZE
            })
            .collect();

        // Find the active indent level (indent level of the current line)
        let active_indent = if current_line > 0 && current_line <= indent_levels.len() {
            indent_levels[current_line - 1]
        } else {
            0
        };

        let scroll_output = scroll_area.show(ui, |ui| {
            let rect = ui.min_rect();
            let painter = ui.painter();

            // Draw current line highlight
            if current_line > 0 {
                let highlight_y = (current_line - 1) as f32 * line_height;
                let highlight_rect = Rect::from_min_size(
                    Pos2::new(rect.left(), rect.top() + highlight_y),
                    Vec2::new(ui.available_width() + 1000.0, line_height),
                );
                painter.rect_filled(highlight_rect, 0.0, colors::CURRENT_LINE_BG);
            }

            // Draw indent guides
            let indent_width = char_width * layout::TAB_SIZE as f32;
            let total_lines = indent_levels.len();

            // Find max indent level to draw
            let max_indent = indent_levels.iter().copied().max().unwrap_or(0);

            for indent in 1..=max_indent {
                let x = rect.left() + (indent as f32 * indent_width)
                    - (indent_width - char_width * 0.5);

                // Draw vertical line segments where this indent level is active
                let mut segment_start: Option<usize> = None;

                for (line_idx, &line_indent) in indent_levels.iter().enumerate() {
                    let is_in_block = line_indent >= indent;

                    match (segment_start, is_in_block) {
                        (None, true) => {
                            segment_start = Some(line_idx);
                        }
                        (Some(start), false) => {
                            // Draw the segment
                            let y_start = rect.top() + (start as f32 * line_height);
                            let y_end = rect.top() + (line_idx as f32 * line_height);

                            let guide_color = if indent == active_indent {
                                colors::INDENT_GUIDE_ACTIVE
                            } else {
                                colors::INDENT_GUIDE
                            };

                            painter.line_segment(
                                [Pos2::new(x, y_start), Pos2::new(x, y_end)],
                                egui::Stroke::new(1.0, guide_color),
                            );
                            segment_start = None;
                        }
                        _ => {}
                    }
                }

                // Draw remaining segment if exists
                if let Some(start) = segment_start {
                    let y_start = rect.top() + (start as f32 * line_height);
                    let y_end = rect.top() + (total_lines as f32 * line_height);

                    let guide_color = if indent == active_indent {
                        colors::INDENT_GUIDE_ACTIVE
                    } else {
                        colors::INDENT_GUIDE
                    };

                    painter.line_segment(
                        [Pos2::new(x, y_start), Pos2::new(x, y_end)],
                        egui::Stroke::new(1.0, guide_color),
                    );
                }
            }

            // Draw find/search match highlights
            let find_matches = self.find_replace.matches.clone();
            let current_match_idx = self.find_replace.current_match;

            for (idx, (start, end)) in find_matches.iter().enumerate() {
                let (start_line, start_col) = offset_to_line_col(&text, *start);
                let (end_line, end_col) = offset_to_line_col(&text, *end);

                // For simplicity, only highlight single-line matches fully
                // Multi-line matches show just first line portion
                if start_line == end_line {
                    let x = rect.left() + (start_col as f32 * char_width);
                    let y = rect.top() + (start_line as f32 * line_height);
                    let width = (end_col - start_col) as f32 * char_width;

                    let match_rect =
                        Rect::from_min_size(Pos2::new(x, y), Vec2::new(width, line_height));

                    let (bg_color, border_color) = if idx == current_match_idx {
                        (colors::FIND_MATCH_CURRENT_BG, colors::FIND_MATCH_BORDER)
                    } else {
                        (colors::FIND_MATCH_BG, colors::FIND_MATCH_BORDER)
                    };

                    painter.rect_filled(match_rect, 2.0, bg_color);
                    if idx == current_match_idx {
                        painter.rect_stroke(match_rect, 2.0, egui::Stroke::new(2.0, border_color));
                    }
                }
            }

            // Draw bracket pair highlights
            let cursor_offset = {
                let file = &self.open_files[self.active_tab];
                file.state.cursor.offset
            };

            if let Some((open_pos, close_pos)) = find_matching_bracket(&text, cursor_offset) {
                // Convert offsets to line/column positions
                let (open_line, open_col) = offset_to_line_col(&text, open_pos);
                let (close_line, close_col) = offset_to_line_col(&text, close_pos);

                // Draw highlight for opening bracket
                let open_x = rect.left() + (open_col as f32 * char_width);
                let open_y = rect.top() + (open_line as f32 * line_height);
                let bracket_rect = Rect::from_min_size(
                    Pos2::new(open_x, open_y),
                    Vec2::new(char_width, line_height),
                );
                painter.rect_filled(bracket_rect, 2.0, colors::BRACKET_MATCH_BG);
                painter.rect_stroke(
                    bracket_rect,
                    2.0,
                    egui::Stroke::new(1.0, colors::BRACKET_MATCH_BORDER),
                );

                // Draw highlight for closing bracket
                let close_x = rect.left() + (close_col as f32 * char_width);
                let close_y = rect.top() + (close_line as f32 * line_height);
                let bracket_rect = Rect::from_min_size(
                    Pos2::new(close_x, close_y),
                    Vec2::new(char_width, line_height),
                );
                painter.rect_filled(bracket_rect, 2.0, colors::BRACKET_MATCH_BG);
                painter.rect_stroke(
                    bracket_rect,
                    2.0,
                    egui::Stroke::new(1.0, colors::BRACKET_MATCH_BORDER),
                );
            }

            let text_edit_id = ui.id().with("editor");
            let response = ui.add(
                TextEdit::multiline(&mut text)
                    .id(text_edit_id)
                    .font(TextStyle::Monospace)
                    .code_editor()
                    .frame(false) // Remove TextEdit's internal frame/margin
                    .margin(Vec2::ZERO) // No margin
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );

            // Try to get cursor position from TextEdit state
            if let Some(state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                if let Some(cursor) = state.cursor.char_range() {
                    let offset = cursor.primary.index;
                    let file = &mut self.open_files[self.active_tab];
                    file.state.cursor = CursorPosition::from_char_offset(&file.buffer, offset);
                }
            }

            response
        });

        // Store scroll offset for gutter sync
        self.editor_scroll_offset = scroll_output.state.offset;

        // Update file state
        let file = &mut self.open_files[self.active_tab];

        // Update visible lines
        let visible_start = (self.editor_scroll_offset.y / line_height).floor() as usize + 1;
        let visible_count = (ui.available_height() / line_height).ceil() as usize;
        file.state.visible_lines = (visible_start, visible_start + visible_count);

        // Auto-closing brackets: detect if a single opening bracket was typed
        let current_char_count = text.chars().count();
        if current_char_count == prev_char_count + 1 {
            // One character was added
            let cursor_offset = file.state.cursor.offset;
            if cursor_offset > 0 && cursor_offset <= text.len() {
                let chars: Vec<char> = text.chars().collect();
                let typed_char = chars.get(cursor_offset.saturating_sub(1)).copied();

                if let Some(ch) = typed_char {
                    let closing = match ch {
                        '(' => Some(')'),
                        '[' => Some(']'),
                        '{' => Some('}'),
                        '"' => Some('"'),
                        '\'' => Some('\''),
                        '`' => Some('`'),
                        _ => None,
                    };

                    if let Some(close_char) = closing {
                        // Insert closing bracket at cursor position
                        let byte_offset = chars[..cursor_offset].iter().collect::<String>().len();
                        text.insert(byte_offset, close_char);
                    }
                }
            }
        }

        // Update buffer if changed
        if text != file.buffer {
            file.buffer = Rope::from_str(&text);
            file.state.is_modified = text != original;
        }
    }

    fn render_welcome_screen(&self, ui: &mut egui::Ui) {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.label(
                    RichText::new("Rust Code Editor")
                        .size(24.0)
                        .color(colors::TEXT_SECONDARY)
                        .strong(),
                );
                ui.add_space(16.0);
                ui.label(
                    RichText::new("Open a file or folder to start editing")
                        .size(fonts::HEADING)
                        .color(Color32::GRAY),
                );
            });
        });
    }

    // === Status Bar ===

    fn render_status_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(layout::STATUS_BAR_HEIGHT)
            .frame(Frame::none())
            .show(ctx, |ui| {
                let info = if let Some(file) = self.open_files.get(self.active_tab) {
                    StatusBarInfo {
                        cursor: file.state.cursor.clone(),
                        language: detect_language(file.extension()).to_string(),
                        encoding: "UTF-8".to_string(),
                        line_ending: if cfg!(windows) { "CRLF" } else { "LF" }.to_string(),
                        total_lines: file.buffer.len_lines(),
                        total_chars: file.buffer.len_chars(),
                    }
                } else {
                    StatusBarInfo::default()
                };

                let file_name = self.open_files.get(self.active_tab).map(|f| f.name());

                StatusBar::new(info).file_name(file_name).show(ui);
            });
    }

    // === File Operations ===

    fn open_folder(&mut self) {
        if let Some(path) = FileDialog::new().pick_folder() {
            self.workspace = Some(path.clone());
            self.tree = vec![FileNode::new(path)];
        }
    }

    fn open_file_dialog(&mut self) {
        if let Some(path) = FileDialog::new().pick_file() {
            self.open_file(path);
        }
    }

    fn open_file(&mut self, path: PathBuf) {
        // Don't open the same file twice
        if let Some(index) = self.open_files.iter().position(|f| f.path == path) {
            self.active_tab = index;
            return;
        }

        let content = std::fs::read_to_string(&path).unwrap_or_default();
        self.open_files.push(OpenFile::new(path, content));
        self.active_tab = self.open_files.len() - 1;
    }

    fn save_current_file(&mut self) {
        if let Some(file) = self.open_files.get_mut(self.active_tab) {
            let content = file.buffer.to_string();
            if std::fs::write(&file.path, &content).is_ok() {
                file.original_content = content;
                file.state.is_modified = false;
            }
        }
    }

    fn close_tab(&mut self, index: usize) {
        self.open_files.remove(index);
        if self.active_tab >= self.open_files.len() && self.active_tab > 0 {
            self.active_tab -= 1;
        }
    }
}
