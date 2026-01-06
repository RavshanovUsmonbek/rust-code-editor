use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    pub fn new(path: PathBuf) -> Self {
        let is_dir = path.is_dir();
        let children = if is_dir {
            Self::load_children(&path)
        } else {
            vec![]
        };
        Self { path, is_dir, children }
    }

    fn load_children(path: &PathBuf) -> Vec<FileNode> {
        let Ok(entries) = fs::read_dir(path) else {
            return vec![];
        };

        let mut children: Vec<FileNode> = entries
            .flatten()
            .map(|entry| FileNode::new(entry.path()))
            .collect();

        // Sort: directories first, then alphabetically by name
        children.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.path.file_name().cmp(&b.path.file_name()),
            }
        });

        children
    }

    pub fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}
