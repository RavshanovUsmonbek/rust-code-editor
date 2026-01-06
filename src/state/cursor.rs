use ropey::Rope;

/// Tracks cursor position within the editor (1-indexed for display)
#[derive(Debug, Clone, Default)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl CursorPosition {
    /// Convert character offset to line/column using ropey::Rope
    pub fn from_char_offset(rope: &Rope, offset: usize) -> Self {
        let safe_offset = offset.min(rope.len_chars().saturating_sub(1).max(0));

        if rope.len_chars() == 0 {
            return Self {
                line: 1,
                column: 1,
                offset: 0,
            };
        }

        let line_idx = rope.char_to_line(safe_offset);
        let line_start = rope.line_to_char(line_idx);
        let column = safe_offset.saturating_sub(line_start) + 1;

        Self {
            line: line_idx + 1,
            column,
            offset: safe_offset,
        }
    }

    /// Format as "Ln X, Col Y" for status bar display
    pub fn display(&self) -> String {
        format!("Ln {}, Col {}", self.line, self.column)
    }
}
