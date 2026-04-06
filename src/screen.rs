use crate::frame::Frame;
use crossterm::{ExecutableCommand, QueueableCommand, cursor, style, terminal};

use std::io::{Stdout, Write};
use std::sync::mpsc::Sender;
use std::{error::Error, io, thread};

pub struct Screen {
    render_tx: Option<Sender<Frame>>,
    render_handle: Option<thread::JoinHandle<()>>,
    stdout: Stdout,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            render_tx: None,
            render_handle: None,
            stdout: io::stdout(),
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        if self.render_handle.is_some() {
            return Result::Ok(()); // Already initializaed
        }

        terminal::enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(cursor::Hide)?;

        // send and receive channels
        let (render_tx, render_rx) = std::sync::mpsc::channel();

        self.render_tx = Some(render_tx);
        self.render_handle = Some(thread::spawn(move || {
            let mut last_frame = Frame::new();
            let mut stdout = io::stdout();
            render(&mut stdout, &last_frame, &last_frame, true);
            loop {
                let curr_frame = match render_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => break,
                };
                render(&mut stdout, &last_frame, &curr_frame, false);
                last_frame = curr_frame;
            }
        }));

        Result::Ok(())
    }

    pub fn update_with_frame(&self, frame: Frame) {
        if let Some(ref tx) = self.render_tx {
            let _ = tx.send(frame);
        } else {
            panic!("init hasn't been called yet")
        }
    }

    pub fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(tx) = self.render_tx.take() {
            drop(tx);
        }
        if let Some(handle) = self.render_handle.take() {
            let _ = handle.join();
        }

        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Result::Ok(())
    }
}

fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout
            .queue(style::SetBackgroundColor(style::Color::Blue))
            .unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout
            .queue(style::SetBackgroundColor(style::Color::Black))
            .unwrap();
    }

    for (x, y, s) in curr_frame.iter() {
        if s != last_frame.get_at(x, y) || force {
            stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
            print!("{}", s);
        }
    }
    stdout.flush().unwrap();
}
