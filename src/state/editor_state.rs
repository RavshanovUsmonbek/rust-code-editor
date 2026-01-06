use super::cursor::CursorPosition;

/// State for a single editor tab
#[derive(Debug, Clone)]
pub struct EditorTabState {
    /// Current cursor position
    pub cursor: CursorPosition,
    /// Whether the file has unsaved changes
    pub is_modified: bool,
    /// Range of visible lines (for minimap viewport indicator)
    pub visible_lines: (usize, usize),
}

impl Default for EditorTabState {
    fn default() -> Self {
        Self {
            cursor: CursorPosition::default(),
            is_modified: false,
            visible_lines: (1, 50),
        }
    }
}
