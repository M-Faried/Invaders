use crate::frame::Frame;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{Stdout, Write};
use std::sync::mpsc::Sender;
use std::{io, thread};

pub struct Screen {
    render_tx: Option<Sender<Frame>>,
    render_handle: Option<thread::JoinHandle<()>>,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            render_tx: None,
            render_handle: None,
        }
    }

    pub fn init(&mut self) {
        if self.render_handle.is_some() {
            return; // Already initializaed
        }

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
    }

    pub fn update_with_frame(&self, frame: Frame) {
        if let Some(ref tx) = self.render_tx {
            let _ = tx.send(frame);
        } else {
            panic!("start hasn't been called yet")
        }
    }

    pub fn clear(&mut self) {
        drop(self.render_tx.take());
        if let Some(handle) = self.render_handle.take() {
            let _ = handle.join();
        }
    }
}

fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
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
