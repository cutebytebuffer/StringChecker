use ratatui::{
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

pub struct App {
    pub logs: Vec<String>,
    pub scroll_up: usize,
}

impl App {
    pub fn new() -> Self {
        Self { logs: Vec::new(), scroll_up: 0 }
    }

    pub fn log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
    }
}

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    crate::scanner::scanner::scan(&mut app);

    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            use crossterm::event::KeyCode;

            if key.is_press() {
                let max = app.logs.len().saturating_sub(1);
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.scroll_up = (app.scroll_up + 1).min(max)
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.scroll_up = app.scroll_up.saturating_sub(1)
                    }
                    KeyCode::PageUp => app.scroll_up = (app.scroll_up + 10).min(max),
                    KeyCode::PageDown => app.scroll_up = app.scroll_up.saturating_sub(10),
                    KeyCode::Home => app.scroll_up = max,
                    KeyCode::End => app.scroll_up = 0,
                    _ => {}
                }
            }
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::default()
        .title("StringChecker (q quit, ↑/↓ PgUp/PgDn Home/End scroll)")
        .borders(Borders::ALL);

    let logs = app.logs.join("\n");

    let visible = area.height.saturating_sub(2) as usize;
    let max_top = app.logs.len().saturating_sub(visible);
    let top = max_top.saturating_sub(app.scroll_up) as u16;

    frame.render_widget(
        Paragraph::new(logs).block(block).scroll((top, 0)),
        area,
    );
}