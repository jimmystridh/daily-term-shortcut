# Keyboard Shortcuts Master

[![Release](https://github.com/jimmystridh/daily-term-shortcut/actions/workflows/release.yml/badge.svg)](https://github.com/jimmystridh/daily-term-shortcut/actions/workflows/release.yml)

An **awesome** interactive TUI application that helps you master keyboard shortcuts across multiple platforms and tools! Built with Rust using the `ratatui` framework.

![screenshot](screenshot.png)

## Features

### Smart Learning System
- **Session-based Learning** - Get fresh, random shortcuts every time you run the app
- **Progress Tracking** - Automatically tracks which shortcuts you've seen
- **Configurable Sessions** - Set how many new shortcuts to show per session (1, 2, 5, or more!)
- **Never Repeat** - Shows only new shortcuts you haven't seen yet until you've mastered them all
- **Visual Progress** - See checkmarks (✓) next to shortcuts you've already learned

### Interactive Navigation
- **Interactive Browsing** - Navigate through shortcuts with vim-style keybindings (j/k) or arrow keys
- **Session Shortcuts Panel** - See your new shortcuts for this session at the top
- **Search Functionality** - Quickly find shortcuts by key combination, description, or category
- **Category Filtering** - Filter shortcuts by specific tools and platforms
- **Beautiful TUI** - Colorful, intuitive terminal interface

### Powerful Configuration
- **Custom Categories** - Choose which tools/platforms you want to learn (e.g., only Vim + Git)
- **Flexible Sessions** - Configure 1-10+ shortcuts per invocation based on your learning pace
- **Persistent State** - Your progress is saved automatically between sessions
- **Easy Reset** - Start over anytime with a single keystroke

### Massive Shortcut Database
280+ keyboard shortcuts across **19 categories**:
- **ZSH** - Terminal shortcuts and command-line editing
- **Vim** - Modal editing, navigation, and commands
- **VS Code** - Editor shortcuts and commands
- **Git** - Version control commands
- **Browser** - Web browser navigation (Chrome, Firefox, etc.)
- **General** - Universal OS shortcuts
- **DevTools** - Browser developer tools
- **Linux** - Linux desktop environment shortcuts
- **macOS** - Mac-specific shortcuts
- **IntelliJ** - IntelliJ IDEA and JetBrains IDEs
- **Sublime** - Sublime Text editor
- **Atom** - Atom editor
- **Tmux** - Terminal multiplexer
- **Terminator** - Terminal emulator
- **Rust** - Cargo commands
- **NPM** - Node package manager commands

## Installation

### Pre-built binaries

Download the pre-built binary for your system from the [releases page](https://github.com/jimmystridh/daily-term-shortcut/releases). Available for:

- Linux (x86_64, ARM64) - Static binaries, no dependencies required
- Windows (x86_64, ARM64)
- macOS (Intel, Apple Silicon)

### Building from source

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
```bash
git clone https://github.com/jimmystridh/daily-term-shortcut
cd daily-term-shortcut
```

3. Build the project:
```bash
cargo build --release
```

4. The binary will be available in `target/release/daily-shortcut`

## Usage

Simply run the application:

```bash
./daily-shortcut
```

### On First Run

The app will:
1. Create a configuration file at `~/.config/keyboard-shortcuts-master/config.toml`
2. Create a state file at `~/.config/keyboard-shortcuts-master/state.json`
3. Show you your first session shortcut(s)!

### Every Time You Run

The app will:
- Pick random shortcuts you haven't seen yet
- Display them in the "Session Shortcuts" panel at the top
- Mark them as "seen" when you quit (press `q`)
- Track your overall progress

### Interactive Controls

Once launched, you can:
- **Browse all shortcuts** using j/k or arrow keys
- **Switch categories** with h/l or arrow keys
- **Search** by pressing `/` and typing your query
- **View settings** by pressing `s` to see your config and progress
- **Toggle session panel** with `d`
- **View help** by pressing `?`
- **Reset progress** by pressing `r` in settings mode

## Configuration

The app stores its configuration in `~/.config/keyboard-shortcuts-master/config.toml`

### Example Configuration

```toml
# Number of new shortcuts to show per session
shortcuts_per_session = 3

# Categories to include (empty array = all categories)
# Uncomment and modify to filter categories:
# enabled_categories = ["Vim", "Git", "VS Code", "ZSH"]
enabled_categories = []

# Show the session shortcuts panel at the top
show_session_shortcuts = true
```

### Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `shortcuts_per_session` | number | 1 | How many new shortcuts to show each time you run the app (1, 2, 3, 5, 10, etc.) |
| `enabled_categories` | array | [] | Which categories to include. Empty = all categories. Example: `["Vim", "Git"]` |
| `show_session_shortcuts` | boolean | true | Whether to display the session shortcuts panel |

### Customization Examples

**Learn 5 new shortcuts per session:**
```toml
shortcuts_per_session = 5
enabled_categories = []
show_session_shortcuts = true
```

**Focus only on Vim and Git:**
```toml
shortcuts_per_session = 2
enabled_categories = ["Vim", "Git"]
show_session_shortcuts = true
```

**Power user mode (10 shortcuts per session):**
```toml
shortcuts_per_session = 10
enabled_categories = []
show_session_shortcuts = true
```

## Keyboard Shortcuts in the App

| Key | Action |
|-----|--------|
| `j` or `↓` | Next shortcut |
| `k` or `↑` | Previous shortcut |
| `h` or `←` | Previous category |
| `l` or `→` | Next category |
| `g` | Jump to first |
| `G` | Jump to last |
| `Ctrl+F` | Page down |
| `Ctrl+B` | Page up |
| `/` | Search |
| `c` | Clear filters |
| `d` | Toggle session shortcuts panel |
| `s` | Settings & info |
| `?` | Help screen |
| `q` | Quit (saves progress) |

## Progress Tracking

The app automatically tracks which shortcuts you've seen in `~/.config/keyboard-shortcuts-master/state.json`

- **Automatic marking**: When you quit the app, session shortcuts are marked as "seen"
- **Visual indicators**: Shortcuts you've seen show a green ✓ checkmark in the list
- **Progress counter**: See your progress (e.g., "Progress: 42/280 seen") in the status bar
- **Reset anytime**: Press `s` for settings, then `r` to reset your progress

## How It Works

1. **First launch**: App shows you N random shortcuts (configured by `shortcuts_per_session`)
2. **Browse & learn**: Explore all shortcuts, search, filter by category
3. **Quit**: Press `q` to exit - your session shortcuts are marked as "seen"
4. **Next launch**: Get N new random shortcuts you haven't seen yet
5. **Cycle complete**: When you've seen all shortcuts, the cycle starts over

## Dependencies

- `ratatui` - Terminal user interface library (v0.29)
- `crossterm` - Terminal manipulation (v0.28)
- `chrono` - Date and time functionality
- `serde` & `serde_json` - Serialization
- `tui-input` - Text input handling
- `toml` - Configuration file parsing
- `dirs` - Cross-platform config directories
- `rand` - Random selection

## Contributing

Contributions are welcome! Here are some ways you can contribute:

1. Add more shortcuts to the database
2. Add support for new categories (Emacs, Eclipse, Photoshop, etc.)
3. Improve the TUI interface
4. Add new learning modes (spaced repetition, quiz mode)
5. Add export functionality

Please feel free to submit issues and pull requests.

## Project Structure

```
src/
  └── main.rs    - Main application with interactive TUI and configuration
~/.config/keyboard-shortcuts-master/
  ├── config.toml  - User configuration
  └── state.json   - Progress tracking state
```

## What Makes This Awesome?

- **Configurable Learning**: Choose exactly how many shortcuts to learn per session (1-10+)
- **Category Selection**: Focus on the tools YOU actually use
- **Smart Progress**: Never see the same shortcut twice until you've learned them all
- **Session-based**: Get new shortcuts every time you run the app
- **Interactive**: Actively browse and discover 280+ shortcuts
- **Persistent State**: Your progress is saved automatically
- **Search Power**: Find what you need instantly
- **Vim-style Navigation**: Familiar, efficient keyboard controls
- **Beautiful Design**: Clean, colorful interface
- **Zero Setup**: Works out of the box with sensible defaults
- **Easy Customization**: Simple TOML config file
- **Blazingly Fast**: Written in Rust for instant responsiveness

## Example Workflows

### Beginner - One per day
```toml
shortcuts_per_session = 1
```
Run the app once a day, learn one new shortcut.

### Intermediate - Morning batch
```toml
shortcuts_per_session = 5
```
Run the app each morning, learn 5 new shortcuts with your coffee.

### Power user - Sprint learning
```toml
shortcuts_per_session = 10
enabled_categories = ["Vim", "VS Code", "Git", "ZSH"]
```
Focus on dev tools, learn 10 at a time, master them quickly.

### Customized - Just what you need
```toml
shortcuts_per_session = 3
enabled_categories = ["Git", "Browser", "General"]
```
Learn only the most practical everyday shortcuts.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/tui-rs-revival/ratatui)
- Inspired by the desire to master keyboard shortcuts across multiple tools
- Community contributions for shortcut database

## Roadmap

- [x] Interactive TUI with keyboard navigation
- [x] Search functionality
- [x] Category filtering
- [x] Session-based learning
- [x] 280+ shortcuts across 19 categories
- [x] Configuration file support
- [x] Progress tracking
- [x] Persistent state
- [x] Configurable shortcuts per session
- [x] Category selection
- [ ] Spaced repetition learning mode
- [ ] Quiz mode to test your knowledge
- [ ] Statistics dashboard
- [ ] Export shortcuts to various formats (PDF, HTML, Markdown)
- [ ] Import custom shortcut files
- [ ] Multiple learning profiles
- [ ] Shortcut difficulty levels

---

⭐ **Star this repository if you find it useful!** ⭐

**Pro tip**: Set `shortcuts_per_session` to match your learning style. Start with 1-2 per day for gradual learning, or go with 5-10 if you're in a hurry to master everything!
