use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum GameCommand {
    None,
    Shoot,
    Exit,
    MoveLeft,
    MoveRight,
}

pub fn get_kb_command() -> GameCommand {
    // reading key presses
    if let Ok(true) = event::poll(Duration::default()) {
        if let Ok(Event::Key(key_event)) = event::read() {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => return GameCommand::Exit,
                KeyCode::Left => return GameCommand::MoveLeft,
                KeyCode::Right => return GameCommand::MoveRight,
                KeyCode::Char(' ') | KeyCode::Enter => return GameCommand::Shoot,
                _ => return GameCommand::None,
            }
        }
    }
    GameCommand::None
}
