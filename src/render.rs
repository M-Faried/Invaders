use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
};

use crate::frame::Frame;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, y, s) in curr_frame.iter() {
        if s != last_frame.get_at(x, y) || force {
            stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
            print!("{}", s);
        }
    }
    stdout.flush().unwrap();
}
