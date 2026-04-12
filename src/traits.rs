use crate::frame::Frame;
use std::time::Duration;

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub trait Tickable {
    fn tick(&mut self, delta: Duration) -> bool;
}
