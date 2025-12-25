use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Debug)]
pub struct Editor {
    pub lines: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    /* ================= RENDER ================= */

    pub fn render(&self, frame: &mut Frame, chunk: Rect) {
        let mut rendered_lines: Vec<Line> = Vec::new();

        for (i, line) in self.lines.iter().enumerate() {
            rendered_lines.push(Line::from(vec![
                Span::styled(
                    format!("{:>3}   ", i + 1),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    line.as_str(),
                    if line.trim_start().starts_with("//") {
                        Style::default().fg(Color::Rgb(120, 160, 120))
                    } else {
                        Style::default().fg(Color::Rgb(220, 220, 220))
                    },
                ),
            ]));
        }

        let paragraph = Paragraph::new(rendered_lines).scroll((0, 0));

        frame.render_widget(paragraph, chunk);

        frame.set_cursor_position(Position {
            x: chunk.x + 6 + self.cursor_x as u16,
            y: chunk.y + self.cursor_y as u16,
        });
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Enter => self.new_line(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            _ => {}
        }
    }

    fn insert_char(&mut self, c: char) {
        self.lines[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
    }

    fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.lines[self.cursor_y].remove(self.cursor_x);
        } else if self.cursor_y > 0 {
            let prev_len = self.lines[self.cursor_y - 1].len();
            let current = self.lines.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = prev_len;
            self.lines[self.cursor_y].push_str(&current);
        }
    }

    fn new_line(&mut self) {
        let rest = self.lines[self.cursor_y].split_off(self.cursor_x);
        self.lines.insert(self.cursor_y + 1, rest);
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    fn move_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor_x < self.lines[self.cursor_y].len() {
            self.cursor_x += 1;
        }
    }

    fn move_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }

    fn move_down(&mut self) {
        if self.cursor_y + 1 < self.lines.len() {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
        }
    }
}
