use std::time::Duration;

use rusty_time::timer::Timer;

use crate::{SHOT_EXPLODING_INTERVAL, SHOT_UPDATE_INTERVAL, frame::Frame, traits::Drawable};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(SHOT_UPDATE_INTERVAL),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        // resetting the timer to wait for the explosion animation before removing the shot.
        self.timer = Timer::from_millis(SHOT_EXPLODING_INTERVAL);
    }

    pub fn is_dead(&self) -> bool {
        // the shot is dead after it has been exlpoding SHOT_EXPLODING_INTERVAL
        // or it reached the end of the screen.
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        let val = if self.exploding { "*" } else { "|" };
        frame.set_at(self.x, self.y, val);
    }
}
