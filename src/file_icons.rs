pub fn get_icon(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    match () {
        // Rust
        _ if lower.ends_with(".rs") => "🦀",

        // Config files
        _ if lower.ends_with(".toml") => "⚙️",
        _ if lower.ends_with(".yaml") || lower.ends_with(".yml") => "⚙️",
        _ if lower.ends_with(".json") => "{ }",
        _ if lower.ends_with(".xml") => "📋",
        _ if lower.ends_with(".ini") || lower.ends_with(".cfg") => "⚙️",
        _ if lower.ends_with(".env") => "🔐",

        // Web
        _ if lower.ends_with(".html") || lower.ends_with(".htm") => "🌐",
        _ if lower.ends_with(".css") || lower.ends_with(".scss") || lower.ends_with(".sass") => "🎨",
        _ if lower.ends_with(".js") => "JS",
        _ if lower.ends_with(".ts") => "TS",
        _ if lower.ends_with(".jsx") || lower.ends_with(".tsx") => "⚛️",
        _ if lower.ends_with(".vue") => "V",
        _ if lower.ends_with(".svelte") => "S",

        // Programming languages
        _ if lower.ends_with(".py") => "🐍",
        _ if lower.ends_with(".go") => "Go",
        _ if lower.ends_with(".java") => "☕",
        _ if lower.ends_with(".kt") || lower.ends_with(".kts") => "K",
        _ if lower.ends_with(".c") || lower.ends_with(".h") => "C",
        _ if lower.ends_with(".cpp") || lower.ends_with(".hpp") || lower.ends_with(".cc") => "C+",
        _ if lower.ends_with(".cs") => "C#",
        _ if lower.ends_with(".rb") => "💎",
        _ if lower.ends_with(".php") => "🐘",
        _ if lower.ends_with(".swift") => "🐦",
        _ if lower.ends_with(".sh") || lower.ends_with(".bash") => "🐚",
        _ if lower.ends_with(".ps1") => "PS",
        _ if lower.ends_with(".sql") => "🗃️",

        // Documentation
        _ if lower.ends_with(".md") || lower.ends_with(".markdown") => "📝",
        _ if lower.ends_with(".txt") => "📄",
        _ if lower.ends_with(".pdf") => "📕",
        _ if lower.ends_with(".doc") || lower.ends_with(".docx") => "📘",

        // Data
        _ if lower.ends_with(".csv") => "📊",
        _ if lower.ends_with(".xlsx") || lower.ends_with(".xls") => "📊",

        // Images
        _ if lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg")
            || lower.ends_with(".gif") || lower.ends_with(".svg") || lower.ends_with(".ico") => "🖼️",

        // Lock files
        _ if lower.ends_with(".lock") => "🔒",
        _ if lower == "cargo.lock" => "🔒",

        // Git
        _ if lower == ".gitignore" || lower == ".gitattributes" => "🔀",

        // Docker
        _ if lower == "dockerfile" || lower.ends_with(".dockerfile") => "🐳",
        _ if lower.starts_with("docker-compose") => "🐳",

        // Default
        _ => "📄",
    }
}

pub const FOLDER_ICON: &str = "📁";
