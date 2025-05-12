use crate::error::AppError;
use crate::layout::AppLayout;
use crate::widget::footer::Footer;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), AppError> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// Draws the application
    fn draw(&self, frame: &mut Frame) {
        let layout = AppLayout::new(frame);
        let footer = Footer {};

        frame.render_widget(footer, layout.outer_areas[1]);
        frame.render_widget(
            Paragraph::new("Projects list").block(Block::new().borders(Borders::ALL)),
            layout.inner_areas[0],
        );
        frame.render_widget(
            Paragraph::new("Logs").block(Block::new().borders(Borders::ALL)),
            layout.inner_areas[1],
        );
    }

    /// Handles events
    fn handle_events(&mut self) -> Result<(), AppError> {
        match event::read()? {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    /// Handles key events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    /// Exits the application
    fn exit(&mut self) {
        self.exit = true;
    }
}
