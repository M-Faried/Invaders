use crate::frame::Frame;
use crate::render::render;
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

    pub fn start(&mut self) {
        if self.render_handle.is_some() {
            return; // Already started
        }

        let (render_tx, render_rx) = std::sync::mpsc::channel();
        self.render_tx = Some(render_tx);
        self.render_handle = Some(thread::spawn(move || {
            let mut last_frame = Frame::new();
            let mut stdout = io::stdout();
            render(&mut stdout, &last_frame, &last_frame, true);
            loop {
                let curr_frame = match render_rx.recv() {
                    Ok(x) => x,
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

    pub fn stop(self) {
        drop(self.render_tx);
        if let Some(handle) = self.render_handle {
            let _ = handle.join().unwrap();
        }
    }
}
