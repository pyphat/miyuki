mod editor;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use editor::Editor;
use ratatui::DefaultTerminal;
use ratatui::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App {
    running: bool,
    editor: Editor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: false,
            editor: Editor::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // top bar
                Constraint::Min(1),    // editor
                Constraint::Length(1), // bottom bar
            ])
            .split(frame.area());

        self.render_title(frame, chunks[0]);
        self.editor.render(frame, chunks[1]);
        self.render_bottom(frame, chunks[2]);
    }

    fn render_title(&self, frame: &mut Frame, chunk: Rect) {
        let left = Line::styled(
            "御行  test.rs",
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30)),
        )
        .left_aligned();

        let right = Line::styled(
            "UTF-8",
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30)),
        )
        .right_aligned();

        frame.render_widget(left, chunk);
        frame.render_widget(right, chunk);
    }

    fn render_bottom(&self, frame: &mut Frame, chunk: Rect) {
        let left = Line::styled(
            "EDIT MODE",
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30)),
        )
        .left_aligned();

        let right = Line::styled(
            "3/3",
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30)),
        )
        .right_aligned();

        frame.render_widget(left, chunk);
        frame.render_widget(right, chunk);
    }

    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                self.running = false
            }
            _ => {
                self.editor.handle_key(key); // 👈 forward input
            }
        }
    }
}
