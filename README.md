# Rust Code Editor

A VSCode-style code editor built with Rust and [egui](https://github.com/emilk/egui).

## Features

- VSCode Dark+ theme
- Syntax highlighting (powered by syntect)
- File explorer with folder tree
- Multiple tabs support
- Minimap navigation
- Line numbers with current line highlight
- Indent guides
- Bracket pair matching
- Auto-closing brackets
- Find and Replace (Ctrl+F / Ctrl+H)
- Cross-platform (Windows, Linux, macOS)

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/RavshanovUsmonbek/rust-code-editor/releases) page:

| Platform | Download |
|----------|----------|
| Windows (64-bit) | `rust_code_editor-windows-x86_64.exe` |
| Linux (64-bit) | `rust_code_editor-linux-x86_64` |
| macOS Intel | `rust_code_editor-macos-x86_64` |
| macOS Apple Silicon | `rust_code_editor-macos-aarch64` |

### Linux

After downloading, make the binary executable:

```bash
chmod +x rust_code_editor-linux-x86_64
./rust_code_editor-linux-x86_64
```

### macOS

After downloading, make the binary executable and remove quarantine:

```bash
chmod +x rust_code_editor-macos-*
xattr -d com.apple.quarantine rust_code_editor-macos-*
./rust_code_editor-macos-aarch64  # or rust_code_editor-macos-x86_64
```

## Build from Source

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

### Linux Dependencies

```bash
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

### Build

```bash
git clone https://github.com/RavshanovUsmonbek/rust-code-editor.git
cd rust_code_editor
cargo build --release
```

The binary will be at `target/release/rust_code_editor` (or `rust_code_editor.exe` on Windows).

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open folder |
| `Ctrl+S` | Save file |
| `Ctrl+F` | Find |
| `Ctrl+H` | Find and Replace |
| `Escape` | Close find panel |
| `Enter` | Find next (in find panel) |
| `Shift+Enter` | Find previous (in find panel) |

## Creating a Release

To create a new release:

1. Update the version in `Cargo.toml`
2. Commit your changes
3. Create and push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions will automatically build binaries for all platforms and create a release.

## License

MIT License - see [LICENSE](LICENSE) for details.
