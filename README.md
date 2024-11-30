# Daily Term Shortcut

A simple, elegant TUI application that helps you learn keyboard shortcuts by showing you a new one each day. Built with Rust using the `ratatui` framework.

[screenshot](screenshot.png)

## Features

- 📅 Shows a different keyboard shortcut each day
- 🎨 Beautiful TUI interface with colored output
- ⌨️ Currently focuses on ZSH shortcuts
- 🚀 Lightweight and fast
- 🔄 Cycles through shortcuts based on day of year

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
```bash
git clone https://github.com/jimmystridh/daily-term-shortcut
cd daily-term-shortcut
```

3. Build the project:
```bash
cargo build --release!
```

4. The binary will be available in `target/release/daily-shortcut`

## Usage

Simply run the application:

```bash
./daily-shortcut
```

The application will display:
- The shortcut category (e.g., ZSH)
- The keyboard shortcut of the day
- A description of what the shortcut does

## Dependencies

- `ratatui`: Terminal user interface library
- `chrono`: Date and time functionality
- `serde`: Serialization framework
- `serde_json`: JSON parsing
- `crossterm`: Terminal manipulation

## Contributing

Contributions are welcome! Here are some ways you can contribute:

1. Add more shortcuts to the JSON database
2. Add support for different categories (vim, emacs, IDE shortcuts, etc.)
3. Improve the TUI interface
4. Add configuration options

Please feel free to submit issues and pull requests.

## Project Structure

```
src/
  └── main.rs    - Main application code
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/tui-rs-revival/ratatui)
- Inspired by the desire to learn keyboard shortcuts incrementally

## Roadmap

- [ ] Add configuration file support
- [ ] Implement shortcut search functionality
- [ ] Add more shortcut categories
- [ ] Add option to mark shortcuts as learned
- [ ] Add spaced repetition learning system

## Author

[Your Name]

---
Feel free to ⭐ this repository if you find it useful!