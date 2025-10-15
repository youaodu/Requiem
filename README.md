# Requiem

A lightweight, high-performance HTTP client built with Rust and iced.

[中文文档](README_CN.md)

## Features

- 🚀 **High Performance**: 50-100MB memory footprint (vs Postman's 300-500MB), <1s startup time
- 💾 **Offline-First**: All data stored locally, Git-friendly text-based storage
- 🎨 **Native UI**: Built with iced framework for native performance
- 🌍 **Internationalization**: Supports English and Chinese
- 📁 **Collection Management**: Hierarchical organization with folders and drag-and-drop
- 🔧 **Environment Variables**: Multi-environment configuration support
- 🎯 **Syntax Highlighting**: Syntax-highlighted request and response bodies

## Implemented Features (v0.1.0)

- ✅ Request editor (method, URL, headers, query params, body)
- ✅ Multiple body formats (JSON, Form-data, Raw text)
- ✅ Collection/folder organization with drag-and-drop
- ✅ Tab-based request management
- ✅ Response viewer with syntax highlighting
- ✅ Environment variables support
- ✅ Authentication (Bearer, Basic, API Key)
- ✅ Context menus for requests/folders
- ✅ Settings dialog
- ✅ Toast notifications
- ✅ Keyboard shortcuts
- ✅ Internationalization (English/Chinese)

## Requirements

- Rust 2021 or higher
- Linux/macOS/Windows
- Chinese font support (optional, for Chinese UI)

## Installation & Usage

### Build from Source

```bash
# Clone the repository
git clone https://github.com/youaodu/Requiem.git
cd Requiem

# Run in development mode
cargo run

# Build release version
cargo build --release

# Run release version
./target/release/requiem
```

### Font Configuration (Optional)

For full Chinese support, ensure Source Han Sans is installed:

**Arch Linux:**
```bash
sudo pacman -S adobe-source-han-sans-otf-fonts
```

**Other Systems:**
Download and install Source Han Sans, or modify the font path in `src/main.rs`.

## Development

### Basic Commands

```bash
# Run in development mode
cargo run

# Enable debug logging
RUST_LOG=debug cargo run

# Code check
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Run tests
cargo test
```

### Debugging Tools

```bash
# Standard debug mode
./debug.sh run

# Full trace mode
./debug.sh trace

# Memory profiling
./debug.sh mem

# CPU profiling
./debug.sh perf
```

## Project Structure

```
src/
├── main.rs              # Entry point
├── i18n.rs              # Internationalization
├── http_client.rs       # HTTP request execution
├── models/              # Core data models
├── app/                 # Application layer
│   ├── state.rs         # Application state
│   ├── message.rs       # Message enum
│   └── update.rs        # State updates
├── ui/                  # User interface
│   ├── view.rs          # Main view
│   ├── request_editor.rs    # Request editor
│   ├── request_list.rs      # Collection sidebar
│   ├── response_viewer.rs   # Response viewer
│   └── components/      # UI components
└── utils/               # Utility functions
```

## Tech Stack

- **Language**: Rust 2021
- **UI Framework**: iced 0.13 (native Elm-architecture GUI)
- **HTTP Client**: reqwest 0.12 + tokio async runtime
- **Syntax Highlighting**: syntect 5

## Performance Targets

- Memory usage: <100MB (target), <200MB (max)
- Startup time: <1s
- Binary size: <30MB (target), <50MB (max)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Issues and Pull Requests are welcome!

## Roadmap

See `idea.md` for the complete feature roadmap and architectural details.
