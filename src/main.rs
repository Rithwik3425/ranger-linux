use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs::{self, DirEntry},
    io::{self, stdout},
    path::{Path, PathBuf},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

struct Navigator {
    levels: Vec<(PathBuf, Vec<DirEntry>, usize, usize)>, // Stack of (path, entries, selected index, scroll offset)
}

impl Navigator {
    fn new() -> Self {
        // Initialize with the home directory as the root
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let entries = Self::read_dir(&home_dir);
        Self {
            levels: vec![(home_dir, entries, 0, 0)], // Initialize with (path, entries, selected, offset)
        }
    }

    fn read_dir(path: &Path) -> Vec<DirEntry> {
        fs::read_dir(path).unwrap().filter_map(Result::ok).collect()
    }

    fn navigate_into(&mut self) {
        if let Some((_, entries, selected, _)) = self.levels.last() {
            if *selected < entries.len() {
                let new_path = entries[*selected].path();
                if new_path.is_dir() {
                    let new_entries = Self::read_dir(&new_path);
                    self.levels.push((new_path, new_entries, 0, 0));
                }
            }
        }
    }

    fn navigate_back(&mut self) {
        if self.levels.len() > 1 {
            self.levels.pop();
        }
    }

    fn scroll_down(&mut self, terminal_height: usize) {
        if let Some((_, entries, selected, offset)) = self.levels.last_mut() {
            if *selected < entries.len() - 1 {
                *selected += 1;
                if *selected >= *offset + terminal_height - 2 {
                    // Adjust offset based on terminal height minus space for borders
                    *offset += 1;
                }
            }
        }
    }

    fn scroll_up(&mut self) {
        if let Some((_, _, selected, offset)) = self.levels.last_mut() {
            if *selected > 0 {
                *selected -= 1;
                if *selected < *offset {
                    *offset -= 1;
                }
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut navigator = Navigator::new();

    loop {
        terminal.draw(|f| {
            let terminal_height = f.size().height as usize; // No unwrap needed here

            // Divide the terminal horizontally based on the current number of levels
            let constraints: Vec<Constraint> = navigator
                .levels
                .iter()
                .map(|_| Constraint::Percentage(100 / navigator.levels.len() as u16))
                .collect();

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(f.size());

            // Render each level as a column
            for (i, (path, entries, selected, offset)) in navigator.levels.iter().enumerate() {
                let visible_entries = entries
                    .iter()
                    .skip(*offset)
                    .take(terminal_height - 2) // Adjust visible entries to fit available height
                    .enumerate()
                    .map(|(j, entry)| {
                        let entry_name = entry.file_name().to_string_lossy().to_string();
                        let indicator = if j + *offset == *selected { ">" } else { " " };
                        Spans::from(vec![
                            Span::raw(format!("{} ", indicator)),
                            Span::styled(
                                entry_name,
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        ])
                    })
                    .collect::<Vec<Spans>>();

                let title = path.file_name().unwrap_or_default().to_string_lossy();
                let paragraph = Paragraph::new(visible_entries).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(Spans::from(vec![Span::styled(
                            title.to_string(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )])),
                );
                f.render_widget(paragraph, chunks[i]);
            }
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break, // Press 'q' to exit
                    KeyCode::Right | KeyCode::Char('l') => navigator.navigate_into(),
                    KeyCode::Left | KeyCode::Char('h') => navigator.navigate_back(),
                    KeyCode::Down | KeyCode::Char('j') => {
                        navigator.scroll_down(terminal.size().unwrap().height as usize)
                    }
                    KeyCode::Up | KeyCode::Char('k') => navigator.scroll_up(),
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
