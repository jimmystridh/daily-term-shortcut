use chrono::Datelike;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Shortcut {
    key: String,
    description: String,
    category: String,
}

#[derive(Deserialize)]
struct ShortcutData {
    shortcuts: Vec<Shortcut>,
}

enum AppMode {
    Normal,
    Search,
    Help,
}

struct App {
    shortcuts: Vec<Shortcut>,
    filtered_shortcuts: Vec<Shortcut>,
    selected: usize,
    category_filter: Option<String>,
    categories: Vec<String>,
    mode: AppMode,
    search_input: Input,
    show_daily: bool,
    daily_shortcut: Shortcut,
}

impl App {
    fn new(shortcuts: Vec<Shortcut>) -> App {
        let daily_shortcut = get_todays_shortcut(&shortcuts).clone();
        let categories = get_categories(&shortcuts);
        let filtered_shortcuts = shortcuts.clone();

        App {
            shortcuts,
            filtered_shortcuts,
            selected: 0,
            category_filter: None,
            categories,
            mode: AppMode::Normal,
            search_input: Input::default(),
            show_daily: true,
            daily_shortcut,
        }
    }

    fn next(&mut self) {
        if !self.filtered_shortcuts.is_empty() {
            self.selected = (self.selected + 1) % self.filtered_shortcuts.len();
        }
    }

    fn previous(&mut self) {
        if !self.filtered_shortcuts.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.filtered_shortcuts.len() - 1;
            }
        }
    }

    fn next_category(&mut self) {
        if let Some(current_filter) = &self.category_filter {
            let current_idx = self.categories.iter().position(|c| c == current_filter).unwrap_or(0);
            let next_idx = (current_idx + 1) % (self.categories.len() + 1);

            if next_idx == self.categories.len() {
                self.category_filter = None;
            } else {
                self.category_filter = Some(self.categories[next_idx].clone());
            }
        } else {
            self.category_filter = Some(self.categories[0].clone());
        }
        self.apply_filters();
    }

    fn previous_category(&mut self) {
        if let Some(current_filter) = &self.category_filter {
            let current_idx = self.categories.iter().position(|c| c == current_filter).unwrap_or(0);
            let prev_idx = if current_idx == 0 {
                self.categories.len()
            } else {
                current_idx - 1
            };

            if prev_idx == self.categories.len() {
                self.category_filter = None;
            } else {
                self.category_filter = Some(self.categories[prev_idx].clone());
            }
        } else {
            self.category_filter = Some(self.categories[self.categories.len() - 1].clone());
        }
        self.apply_filters();
    }

    fn apply_filters(&mut self) {
        let search_term = self.search_input.value().to_lowercase();

        self.filtered_shortcuts = self.shortcuts
            .iter()
            .filter(|s| {
                let matches_category = if let Some(ref filter) = self.category_filter {
                    &s.category == filter
                } else {
                    true
                };

                let matches_search = if search_term.is_empty() {
                    true
                } else {
                    s.key.to_lowercase().contains(&search_term) ||
                    s.description.to_lowercase().contains(&search_term) ||
                    s.category.to_lowercase().contains(&search_term)
                };

                matches_category && matches_search
            })
            .cloned()
            .collect();

        if self.selected >= self.filtered_shortcuts.len() && !self.filtered_shortcuts.is_empty() {
            self.selected = self.filtered_shortcuts.len() - 1;
        }
    }

    fn page_down(&mut self, page_size: usize) {
        if !self.filtered_shortcuts.is_empty() {
            self.selected = (self.selected + page_size).min(self.filtered_shortcuts.len() - 1);
        }
    }

    fn page_up(&mut self, page_size: usize) {
        if self.selected >= page_size {
            self.selected -= page_size;
        } else {
            self.selected = 0;
        }
    }
}

fn get_todays_shortcut(shortcuts: &[Shortcut]) -> &Shortcut {
    let today = chrono::Local::now();
    let day_of_year = today.ordinal0() as usize;
    let index = day_of_year % shortcuts.len();
    &shortcuts[index]
}

fn get_categories(shortcuts: &[Shortcut]) -> Vec<String> {
    let mut categories: Vec<String> = shortcuts
        .iter()
        .map(|s| s.category.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    categories.sort();
    categories
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load shortcuts from the embedded JSON with LOTS of shortcuts
    let shortcuts_json = r#"{
        "shortcuts": [
            {"key": "Ctrl+A", "description": "Move to beginning of line", "category": "ZSH"},
            {"key": "Ctrl+E", "description": "Move to end of line", "category": "ZSH"},
            {"key": "Ctrl+K", "description": "Delete from cursor to end of line", "category": "ZSH"},
            {"key": "Ctrl+U", "description": "Delete from cursor to beginning of line", "category": "ZSH"},
            {"key": "Ctrl+W", "description": "Delete word backwards", "category": "ZSH"},
            {"key": "Ctrl+Y", "description": "Paste last deleted text", "category": "ZSH"},
            {"key": "Ctrl+L", "description": "Clear screen", "category": "ZSH"},
            {"key": "Ctrl+R", "description": "Reverse search command history", "category": "ZSH"},
            {"key": "Ctrl+S", "description": "Forward search command history", "category": "ZSH"},
            {"key": "Ctrl+T", "description": "Transpose characters", "category": "ZSH"},
            {"key": "Ctrl+X,A", "description": "Expand alias under cursor", "category": "ZSH"},
            {"key": "Ctrl+X,Ctrl+E", "description": "Edit current command in $EDITOR", "category": "ZSH"},
            {"key": "Ctrl+X,Ctrl+X", "description": "Toggle between start of line and current position", "category": "ZSH"},
            {"key": "Ctrl+X,*", "description": "Expand word to all possible matches", "category": "ZSH"},
            {"key": "Ctrl+X,?", "description": "Get information about key bindings", "category": "ZSH"},
            {"key": "Ctrl+X,g", "description": "List possible glob expansions", "category": "ZSH"},
            {"key": "Alt+F", "description": "Move forward one word", "category": "ZSH"},
            {"key": "Alt+B", "description": "Move backward one word", "category": "ZSH"},
            {"key": "Alt+D", "description": "Delete word forward", "category": "ZSH"},
            {"key": "Alt+Backspace", "description": "Delete word backward", "category": "ZSH"},
            {"key": "Alt+.", "description": "Insert last argument from previous command", "category": "ZSH"},
            {"key": "Alt+T", "description": "Transpose words", "category": "ZSH"},
            {"key": "Alt+U", "description": "Uppercase word", "category": "ZSH"},
            {"key": "Alt+L", "description": "Lowercase word", "category": "ZSH"},
            {"key": "Alt+C", "description": "Capitalize word", "category": "ZSH"},

            {"key": "i", "description": "Enter insert mode before cursor", "category": "Vim"},
            {"key": "a", "description": "Enter insert mode after cursor", "category": "Vim"},
            {"key": "I", "description": "Enter insert mode at beginning of line", "category": "Vim"},
            {"key": "A", "description": "Enter insert mode at end of line", "category": "Vim"},
            {"key": "o", "description": "Open new line below and enter insert mode", "category": "Vim"},
            {"key": "O", "description": "Open new line above and enter insert mode", "category": "Vim"},
            {"key": "h", "description": "Move left", "category": "Vim"},
            {"key": "j", "description": "Move down", "category": "Vim"},
            {"key": "k", "description": "Move up", "category": "Vim"},
            {"key": "l", "description": "Move right", "category": "Vim"},
            {"key": "w", "description": "Move forward to start of word", "category": "Vim"},
            {"key": "b", "description": "Move backward to start of word", "category": "Vim"},
            {"key": "e", "description": "Move forward to end of word", "category": "Vim"},
            {"key": "0", "description": "Move to beginning of line", "category": "Vim"},
            {"key": "$", "description": "Move to end of line", "category": "Vim"},
            {"key": "gg", "description": "Move to first line", "category": "Vim"},
            {"key": "G", "description": "Move to last line", "category": "Vim"},
            {"key": "dd", "description": "Delete line", "category": "Vim"},
            {"key": "yy", "description": "Yank (copy) line", "category": "Vim"},
            {"key": "p", "description": "Paste after cursor", "category": "Vim"},
            {"key": "P", "description": "Paste before cursor", "category": "Vim"},
            {"key": "u", "description": "Undo", "category": "Vim"},
            {"key": "Ctrl+R", "description": "Redo", "category": "Vim"},
            {"key": "x", "description": "Delete character", "category": "Vim"},
            {"key": "r", "description": "Replace character", "category": "Vim"},
            {"key": "ci\"", "description": "Change inside quotes", "category": "Vim"},
            {"key": "ci(", "description": "Change inside parentheses", "category": "Vim"},
            {"key": "ci{", "description": "Change inside braces", "category": "Vim"},
            {"key": "di\"", "description": "Delete inside quotes", "category": "Vim"},
            {"key": "vi\"", "description": "Visual select inside quotes", "category": "Vim"},
            {"key": "/pattern", "description": "Search forward for pattern", "category": "Vim"},
            {"key": "?pattern", "description": "Search backward for pattern", "category": "Vim"},
            {"key": "n", "description": "Repeat search forward", "category": "Vim"},
            {"key": "N", "description": "Repeat search backward", "category": "Vim"},
            {"key": ":%s/old/new/g", "description": "Replace all occurrences in file", "category": "Vim"},
            {"key": ":w", "description": "Save file", "category": "Vim"},
            {"key": ":q", "description": "Quit", "category": "Vim"},
            {"key": ":wq", "description": "Save and quit", "category": "Vim"},
            {"key": ":q!", "description": "Quit without saving", "category": "Vim"},
            {"key": "v", "description": "Enter visual mode", "category": "Vim"},
            {"key": "V", "description": "Enter visual line mode", "category": "Vim"},
            {"key": "Ctrl+V", "description": "Enter visual block mode", "category": "Vim"},
            {"key": ">>", "description": "Indent line", "category": "Vim"},
            {"key": "<<", "description": "Unindent line", "category": "Vim"},
            {"key": ".", "description": "Repeat last command", "category": "Vim"},
            {"key": "*", "description": "Search for word under cursor", "category": "Vim"},
            {"key": "f{char}", "description": "Jump to next occurrence of char", "category": "Vim"},
            {"key": "F{char}", "description": "Jump to previous occurrence of char", "category": "Vim"},
            {"key": "t{char}", "description": "Jump till next occurrence of char", "category": "Vim"},
            {"key": "%", "description": "Jump to matching bracket", "category": "Vim"},

            {"key": "Ctrl+P", "description": "Quick open file", "category": "VS Code"},
            {"key": "Ctrl+Shift+P", "description": "Command palette", "category": "VS Code"},
            {"key": "Ctrl+B", "description": "Toggle sidebar", "category": "VS Code"},
            {"key": "Ctrl+`", "description": "Toggle terminal", "category": "VS Code"},
            {"key": "Ctrl+/", "description": "Toggle line comment", "category": "VS Code"},
            {"key": "Ctrl+Shift+/", "description": "Toggle block comment", "category": "VS Code"},
            {"key": "Alt+Up", "description": "Move line up", "category": "VS Code"},
            {"key": "Alt+Down", "description": "Move line down", "category": "VS Code"},
            {"key": "Shift+Alt+Down", "description": "Copy line down", "category": "VS Code"},
            {"key": "Shift+Alt+Up", "description": "Copy line up", "category": "VS Code"},
            {"key": "Ctrl+D", "description": "Add selection to next find match", "category": "VS Code"},
            {"key": "Ctrl+Shift+L", "description": "Select all occurrences of find match", "category": "VS Code"},
            {"key": "Ctrl+F", "description": "Find", "category": "VS Code"},
            {"key": "Ctrl+H", "description": "Replace", "category": "VS Code"},
            {"key": "Ctrl+Shift+F", "description": "Find in files", "category": "VS Code"},
            {"key": "Ctrl+Shift+H", "description": "Replace in files", "category": "VS Code"},
            {"key": "F12", "description": "Go to definition", "category": "VS Code"},
            {"key": "Alt+F12", "description": "Peek definition", "category": "VS Code"},
            {"key": "Shift+F12", "description": "Show references", "category": "VS Code"},
            {"key": "F2", "description": "Rename symbol", "category": "VS Code"},
            {"key": "Ctrl+Space", "description": "Trigger suggest", "category": "VS Code"},
            {"key": "Ctrl+Shift+Space", "description": "Trigger parameter hints", "category": "VS Code"},
            {"key": "Ctrl+.", "description": "Quick fix", "category": "VS Code"},
            {"key": "Ctrl+Tab", "description": "Switch between open files", "category": "VS Code"},
            {"key": "Ctrl+K Ctrl+F", "description": "Format selection", "category": "VS Code"},
            {"key": "Shift+Alt+F", "description": "Format document", "category": "VS Code"},
            {"key": "Ctrl+K V", "description": "Open markdown preview to side", "category": "VS Code"},
            {"key": "Ctrl+Shift+E", "description": "Show explorer", "category": "VS Code"},
            {"key": "Ctrl+Shift+G", "description": "Show source control", "category": "VS Code"},
            {"key": "Ctrl+Shift+D", "description": "Show debug", "category": "VS Code"},
            {"key": "Ctrl+Shift+X", "description": "Show extensions", "category": "VS Code"},

            {"key": "git status", "description": "Show working tree status", "category": "Git"},
            {"key": "git add .", "description": "Stage all changes", "category": "Git"},
            {"key": "git add -p", "description": "Stage changes interactively", "category": "Git"},
            {"key": "git commit -m", "description": "Commit with message", "category": "Git"},
            {"key": "git commit --amend", "description": "Amend last commit", "category": "Git"},
            {"key": "git push", "description": "Push to remote", "category": "Git"},
            {"key": "git pull", "description": "Pull from remote", "category": "Git"},
            {"key": "git fetch", "description": "Fetch from remote", "category": "Git"},
            {"key": "git branch", "description": "List branches", "category": "Git"},
            {"key": "git branch -d", "description": "Delete branch", "category": "Git"},
            {"key": "git checkout", "description": "Switch branch", "category": "Git"},
            {"key": "git checkout -b", "description": "Create and switch to new branch", "category": "Git"},
            {"key": "git merge", "description": "Merge branch", "category": "Git"},
            {"key": "git rebase", "description": "Rebase current branch", "category": "Git"},
            {"key": "git log", "description": "Show commit logs", "category": "Git"},
            {"key": "git log --oneline", "description": "Show compact commit logs", "category": "Git"},
            {"key": "git log --graph", "description": "Show commit graph", "category": "Git"},
            {"key": "git diff", "description": "Show changes", "category": "Git"},
            {"key": "git diff --staged", "description": "Show staged changes", "category": "Git"},
            {"key": "git stash", "description": "Stash changes", "category": "Git"},
            {"key": "git stash pop", "description": "Apply and remove stash", "category": "Git"},
            {"key": "git reset HEAD~", "description": "Undo last commit keeping changes", "category": "Git"},
            {"key": "git reset --hard", "description": "Reset to last commit discarding changes", "category": "Git"},
            {"key": "git cherry-pick", "description": "Apply commit to current branch", "category": "Git"},
            {"key": "git blame", "description": "Show who changed each line", "category": "Git"},

            {"key": "Ctrl+Shift+T", "description": "Reopen closed tab", "category": "Browser"},
            {"key": "Ctrl+Tab", "description": "Next tab", "category": "Browser"},
            {"key": "Ctrl+Shift+Tab", "description": "Previous tab", "category": "Browser"},
            {"key": "Ctrl+W", "description": "Close tab", "category": "Browser"},
            {"key": "Ctrl+T", "description": "New tab", "category": "Browser"},
            {"key": "Ctrl+N", "description": "New window", "category": "Browser"},
            {"key": "Ctrl+Shift+N", "description": "New incognito window", "category": "Browser"},
            {"key": "Ctrl+L", "description": "Focus address bar", "category": "Browser"},
            {"key": "Ctrl+R", "description": "Reload page", "category": "Browser"},
            {"key": "Ctrl+Shift+R", "description": "Hard reload page", "category": "Browser"},
            {"key": "F12", "description": "Open developer tools", "category": "Browser"},
            {"key": "Ctrl+Shift+I", "description": "Open developer tools", "category": "Browser"},
            {"key": "Ctrl+Shift+J", "description": "Open console", "category": "Browser"},
            {"key": "Ctrl+F", "description": "Find in page", "category": "Browser"},
            {"key": "Ctrl+H", "description": "Open history", "category": "Browser"},
            {"key": "Ctrl+D", "description": "Bookmark current page", "category": "Browser"},
            {"key": "Ctrl+Shift+D", "description": "Bookmark all tabs", "category": "Browser"},
            {"key": "Alt+Left", "description": "Go back", "category": "Browser"},
            {"key": "Alt+Right", "description": "Go forward", "category": "Browser"},
            {"key": "Space", "description": "Scroll down", "category": "Browser"},
            {"key": "Shift+Space", "description": "Scroll up", "category": "Browser"},

            {"key": "Ctrl+C", "description": "Copy", "category": "General"},
            {"key": "Ctrl+V", "description": "Paste", "category": "General"},
            {"key": "Ctrl+X", "description": "Cut", "category": "General"},
            {"key": "Ctrl+Z", "description": "Undo", "category": "General"},
            {"key": "Ctrl+Y", "description": "Redo", "category": "General"},
            {"key": "Ctrl+A", "description": "Select all", "category": "General"},
            {"key": "Ctrl+S", "description": "Save", "category": "General"},
            {"key": "Ctrl+P", "description": "Print", "category": "General"},
            {"key": "Ctrl+F", "description": "Find", "category": "General"},
            {"key": "Alt+Tab", "description": "Switch between applications", "category": "General"},
            {"key": "Alt+F4", "description": "Close window", "category": "General"},
            {"key": "Win+D", "description": "Show desktop", "category": "General"},
            {"key": "Win+E", "description": "Open file explorer", "category": "General"},
            {"key": "Win+L", "description": "Lock computer", "category": "General"},
            {"key": "Win+Tab", "description": "Task view", "category": "General"},
            {"key": "PrtScn", "description": "Screenshot", "category": "General"},

            {"key": "Ctrl+Shift+C", "description": "Inspect element", "category": "DevTools"},
            {"key": "Ctrl+[", "description": "Previous panel", "category": "DevTools"},
            {"key": "Ctrl+]", "description": "Next panel", "category": "DevTools"},
            {"key": "Ctrl+F", "description": "Search in current file", "category": "DevTools"},
            {"key": "Ctrl+Shift+F", "description": "Search in all files", "category": "DevTools"},
            {"key": "Ctrl+G", "description": "Go to line", "category": "DevTools"},
            {"key": "Ctrl+O", "description": "Go to source file", "category": "DevTools"},
            {"key": "Ctrl+P", "description": "Go to command", "category": "DevTools"},
            {"key": "F8", "description": "Resume script execution", "category": "DevTools"},
            {"key": "F10", "description": "Step over", "category": "DevTools"},
            {"key": "F11", "description": "Step into", "category": "DevTools"},
            {"key": "Shift+F11", "description": "Step out", "category": "DevTools"},

            {"key": "Ctrl+Alt+T", "description": "Open terminal", "category": "Linux"},
            {"key": "Ctrl+Alt+L", "description": "Lock screen", "category": "Linux"},
            {"key": "Ctrl+Alt+Del", "description": "System monitor", "category": "Linux"},
            {"key": "Alt+F2", "description": "Run command", "category": "Linux"},
            {"key": "Super+A", "description": "Show applications", "category": "Linux"},
            {"key": "Super+S", "description": "Show overview", "category": "Linux"},
            {"key": "Alt+F7", "description": "Move window", "category": "Linux"},
            {"key": "Alt+F8", "description": "Resize window", "category": "Linux"},
            {"key": "Alt+F9", "description": "Minimize window", "category": "Linux"},
            {"key": "Alt+F10", "description": "Maximize window", "category": "Linux"},

            {"key": "Cmd+C", "description": "Copy", "category": "macOS"},
            {"key": "Cmd+V", "description": "Paste", "category": "macOS"},
            {"key": "Cmd+X", "description": "Cut", "category": "macOS"},
            {"key": "Cmd+Z", "description": "Undo", "category": "macOS"},
            {"key": "Cmd+Shift+Z", "description": "Redo", "category": "macOS"},
            {"key": "Cmd+A", "description": "Select all", "category": "macOS"},
            {"key": "Cmd+S", "description": "Save", "category": "macOS"},
            {"key": "Cmd+Q", "description": "Quit application", "category": "macOS"},
            {"key": "Cmd+W", "description": "Close window", "category": "macOS"},
            {"key": "Cmd+Space", "description": "Spotlight search", "category": "macOS"},
            {"key": "Cmd+Tab", "description": "Switch applications", "category": "macOS"},
            {"key": "Cmd+`", "description": "Switch windows of same application", "category": "macOS"},
            {"key": "Cmd+H", "description": "Hide window", "category": "macOS"},
            {"key": "Cmd+M", "description": "Minimize window", "category": "macOS"},
            {"key": "Cmd+,", "description": "Open preferences", "category": "macOS"},
            {"key": "Cmd+Shift+3", "description": "Screenshot entire screen", "category": "macOS"},
            {"key": "Cmd+Shift+4", "description": "Screenshot selection", "category": "macOS"},
            {"key": "Cmd+Shift+5", "description": "Screenshot/recording options", "category": "macOS"},

            {"key": "Ctrl+Shift+N", "description": "New file", "category": "IntelliJ"},
            {"key": "Ctrl+Shift+A", "description": "Find action", "category": "IntelliJ"},
            {"key": "Shift+Shift", "description": "Search everywhere", "category": "IntelliJ"},
            {"key": "Ctrl+N", "description": "Go to class", "category": "IntelliJ"},
            {"key": "Ctrl+Shift+N", "description": "Go to file", "category": "IntelliJ"},
            {"key": "Ctrl+Alt+Shift+N", "description": "Go to symbol", "category": "IntelliJ"},
            {"key": "Ctrl+B", "description": "Go to declaration", "category": "IntelliJ"},
            {"key": "Ctrl+Alt+B", "description": "Go to implementation", "category": "IntelliJ"},
            {"key": "Ctrl+U", "description": "Go to super method", "category": "IntelliJ"},
            {"key": "Ctrl+E", "description": "Recent files", "category": "IntelliJ"},
            {"key": "Ctrl+Shift+E", "description": "Recent locations", "category": "IntelliJ"},
            {"key": "Alt+F7", "description": "Find usages", "category": "IntelliJ"},
            {"key": "Ctrl+F12", "description": "File structure", "category": "IntelliJ"},
            {"key": "Ctrl+H", "description": "Type hierarchy", "category": "IntelliJ"},
            {"key": "Ctrl+Alt+H", "description": "Call hierarchy", "category": "IntelliJ"},

            {"key": "Ctrl+K Ctrl+B", "description": "Toggle sidebar", "category": "Sublime"},
            {"key": "Ctrl+K Ctrl+M", "description": "Toggle minimap", "category": "Sublime"},
            {"key": "Ctrl+Shift+P", "description": "Command palette", "category": "Sublime"},
            {"key": "Ctrl+P", "description": "Go to file", "category": "Sublime"},
            {"key": "Ctrl+R", "description": "Go to symbol", "category": "Sublime"},
            {"key": "Ctrl+G", "description": "Go to line", "category": "Sublime"},
            {"key": "Ctrl+Shift+D", "description": "Duplicate line", "category": "Sublime"},
            {"key": "Ctrl+Shift+K", "description": "Delete line", "category": "Sublime"},
            {"key": "Ctrl+]", "description": "Indent", "category": "Sublime"},
            {"key": "Ctrl+[", "description": "Unindent", "category": "Sublime"},
            {"key": "Ctrl+L", "description": "Select line", "category": "Sublime"},
            {"key": "Ctrl+D", "description": "Select word (repeat for next)", "category": "Sublime"},
            {"key": "Alt+F3", "description": "Select all occurrences", "category": "Sublime"},

            {"key": "Ctrl+Shift+P", "description": "Command palette", "category": "Atom"},
            {"key": "Ctrl+T", "description": "Fuzzy find file", "category": "Atom"},
            {"key": "Ctrl+B", "description": "Find open file", "category": "Atom"},
            {"key": "Ctrl+Shift+B", "description": "Find modified file", "category": "Atom"},
            {"key": "Ctrl+Alt+R", "description": "Reload window", "category": "Atom"},
            {"key": "Ctrl+Shift+L", "description": "Select grammar", "category": "Atom"},
            {"key": "Ctrl+Shift+M", "description": "Preview markdown", "category": "Atom"},
            {"key": "Ctrl+Alt+I", "description": "Toggle developer tools", "category": "Atom"},

            {"key": "tmux", "description": "Start new session", "category": "Tmux"},
            {"key": "Ctrl+B C", "description": "Create new window", "category": "Tmux"},
            {"key": "Ctrl+B N", "description": "Next window", "category": "Tmux"},
            {"key": "Ctrl+B P", "description": "Previous window", "category": "Tmux"},
            {"key": "Ctrl+B D", "description": "Detach session", "category": "Tmux"},
            {"key": "Ctrl+B %", "description": "Split vertically", "category": "Tmux"},
            {"key": "Ctrl+B \"", "description": "Split horizontally", "category": "Tmux"},
            {"key": "Ctrl+B Arrow", "description": "Navigate panes", "category": "Tmux"},
            {"key": "Ctrl+B X", "description": "Kill pane", "category": "Tmux"},
            {"key": "Ctrl+B [", "description": "Enter scroll mode", "category": "Tmux"},

            {"key": "Ctrl+Shift+I", "description": "Split horizontally", "category": "Terminator"},
            {"key": "Ctrl+Shift+O", "description": "Split vertically", "category": "Terminator"},
            {"key": "Ctrl+Shift+W", "description": "Close terminal", "category": "Terminator"},
            {"key": "Ctrl+Shift+E", "description": "Close all terminals in window", "category": "Terminator"},
            {"key": "Alt+Arrow", "description": "Navigate terminals", "category": "Terminator"},
            {"key": "Ctrl+Shift+X", "description": "Toggle terminal zoom", "category": "Terminator"},

            {"key": "cargo build", "description": "Build the project", "category": "Rust"},
            {"key": "cargo run", "description": "Build and run the project", "category": "Rust"},
            {"key": "cargo test", "description": "Run tests", "category": "Rust"},
            {"key": "cargo check", "description": "Check code without building", "category": "Rust"},
            {"key": "cargo clippy", "description": "Run linter", "category": "Rust"},
            {"key": "cargo fmt", "description": "Format code", "category": "Rust"},
            {"key": "cargo doc --open", "description": "Build and open documentation", "category": "Rust"},
            {"key": "cargo clean", "description": "Clean build artifacts", "category": "Rust"},
            {"key": "cargo update", "description": "Update dependencies", "category": "Rust"},

            {"key": "npm install", "description": "Install dependencies", "category": "NPM"},
            {"key": "npm start", "description": "Start application", "category": "NPM"},
            {"key": "npm test", "description": "Run tests", "category": "NPM"},
            {"key": "npm run build", "description": "Build project", "category": "NPM"},
            {"key": "npm run dev", "description": "Start development server", "category": "NPM"},
            {"key": "npm audit", "description": "Security audit", "category": "NPM"},
            {"key": "npm outdated", "description": "Check for outdated packages", "category": "NPM"},
            {"key": "npm update", "description": "Update packages", "category": "NPM"}
        ]
    }"#;

    let shortcut_data: ShortcutData = serde_json::from_str(shortcuts_json)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(shortcut_data.shortcuts);
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Search => {
                        match key.code {
                            KeyCode::Esc => {
                                app.mode = AppMode::Normal;
                            }
                            KeyCode::Enter => {
                                app.mode = AppMode::Normal;
                            }
                            _ => {
                                app.search_input.handle_event(&Event::Key(key));
                                app.apply_filters();
                            }
                        }
                    }
                    AppMode::Help => {
                        if key.code == KeyCode::Char('?') || key.code == KeyCode::Esc {
                            app.mode = AppMode::Normal;
                        }
                    }
                    AppMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('j') | KeyCode::Down => app.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.previous(),
                            KeyCode::Char('h') | KeyCode::Left => app.previous_category(),
                            KeyCode::Char('l') | KeyCode::Right => app.next_category(),
                            KeyCode::Char('d') => app.show_daily = !app.show_daily,
                            KeyCode::Char('/') => {
                                app.mode = AppMode::Search;
                            }
                            KeyCode::Char('c') => {
                                app.category_filter = None;
                                app.apply_filters();
                            }
                            KeyCode::Char('?') => {
                                app.mode = AppMode::Help;
                            }
                            KeyCode::PageDown | KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.page_down(10);
                            }
                            KeyCode::PageUp | KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                app.page_up(10);
                            }
                            KeyCode::Home | KeyCode::Char('g') => {
                                app.selected = 0;
                            }
                            KeyCode::End | KeyCode::Char('G') => {
                                if !app.filtered_shortcuts.is_empty() {
                                    app.selected = app.filtered_shortcuts.len() - 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(1),  // Stats/filter
            Constraint::Min(0),     // Content
            Constraint::Length(3),  // Help
        ])
        .split(size);

    // Title
    let title = Paragraph::new("Keyboard Shortcuts Master")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Stats and filter info
    let filter_text = if let Some(ref cat) = app.category_filter {
        format!("Category: {} | ", cat)
    } else {
        "Category: All | ".to_string()
    };

    let stats = Paragraph::new(format!(
        "{}Showing {} of {} shortcuts",
        filter_text,
        app.filtered_shortcuts.len(),
        app.shortcuts.len()
    ))
    .style(Style::default().fg(Color::Yellow));
    f.render_widget(stats, chunks[1]);

    // Content area
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if app.show_daily {
            vec![Constraint::Length(6), Constraint::Min(0)]
        } else {
            vec![Constraint::Min(0)]
        })
        .split(chunks[2]);

    // Daily shortcut box
    if app.show_daily {
        let daily = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Today's Shortcut: ", Style::default().fg(Color::Yellow)),
                Span::styled(&app.daily_shortcut.key, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Cyan)),
                Span::raw(&app.daily_shortcut.description),
            ]),
            Line::from(vec![
                Span::styled("Category: ", Style::default().fg(Color::Magenta)),
                Span::raw(&app.daily_shortcut.category),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Daily Shortcut (toggle with 'd')"))
        .style(Style::default().fg(Color::White));
        f.render_widget(daily, content_chunks[0]);
    }

    let list_area = if app.show_daily { content_chunks[1] } else { content_chunks[0] };

    // Show appropriate content based on mode
    match app.mode {
        AppMode::Help => {
            let help_text = vec![
                Line::from(""),
                Line::from(vec![Span::styled("Keyboard Shortcuts", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("j/k or ↓/↑", Style::default().fg(Color::Green)),
                    Span::raw("    - Navigate shortcuts"),
                ]),
                Line::from(vec![
                    Span::styled("h/l or ←/→", Style::default().fg(Color::Green)),
                    Span::raw("    - Switch categories"),
                ]),
                Line::from(vec![
                    Span::styled("g/G", Style::default().fg(Color::Green)),
                    Span::raw("           - Jump to first/last"),
                ]),
                Line::from(vec![
                    Span::styled("Ctrl+F/B", Style::default().fg(Color::Green)),
                    Span::raw("      - Page down/up"),
                ]),
                Line::from(vec![
                    Span::styled("/", Style::default().fg(Color::Green)),
                    Span::raw("             - Search"),
                ]),
                Line::from(vec![
                    Span::styled("c", Style::default().fg(Color::Green)),
                    Span::raw("             - Clear category filter"),
                ]),
                Line::from(vec![
                    Span::styled("d", Style::default().fg(Color::Green)),
                    Span::raw("             - Toggle daily shortcut"),
                ]),
                Line::from(vec![
                    Span::styled("?", Style::default().fg(Color::Green)),
                    Span::raw("             - Toggle this help"),
                ]),
                Line::from(vec![
                    Span::styled("q", Style::default().fg(Color::Green)),
                    Span::raw("             - Quit"),
                ]),
                Line::from(""),
            ];

            let help = Paragraph::new(help_text)
                .block(Block::default().borders(Borders::ALL).title("Help"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(help, list_area);
        }
        AppMode::Search => {
            let search_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(list_area);

            let width = search_chunks[0].width.max(3) - 3;
            let scroll = app.search_input.visual_scroll(width as usize);

            let input = Paragraph::new(app.search_input.value())
                .style(Style::default().fg(Color::Yellow))
                .scroll((0, scroll as u16))
                .block(Block::default().borders(Borders::ALL).title("Search (ESC to exit, Enter to confirm)"));
            f.render_widget(input, search_chunks[0]);

            f.set_cursor_position((
                search_chunks[0].x + ((app.search_input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                search_chunks[0].y + 1,
            ));

            render_shortcut_list(f, app, search_chunks[1]);
        }
        AppMode::Normal => {
            render_shortcut_list(f, app, list_area);
        }
    }

    // Help bar
    let help_text = match app.mode {
        AppMode::Search => "ESC: Exit Search | Enter: Confirm",
        AppMode::Help => "?: Close Help",
        AppMode::Normal => "j/k: Navigate | h/l: Categories | /: Search | ?: Help | q: Quit",
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[3]);
}

fn render_shortcut_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_shortcuts
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let content = vec![Line::from(vec![
                Span::styled(
                    format!("{:20}", s.key),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" │ "),
                Span::styled(
                    format!("{:12}", s.category),
                    Style::default().fg(Color::Magenta),
                ),
                Span::raw(" │ "),
                Span::styled(&s.description, Style::default().fg(Color::White)),
            ])];

            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "Shortcuts (showing {}/{})",
            app.filtered_shortcuts.len(),
            app.shortcuts.len()
        )))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}
