use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{INVADERS_HIGHT, INVADERS_WIDTH, NUM_COLS, NUM_ROWS, frame::Frame, traits::Drawable};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

enum Direction {
    Left,
    Right,
}
impl Direction {
    fn get_step_value(&self) -> isize {
        match self {
            Self::Left => -1,
            _ => 1,
        }
    }
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: Direction,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..INVADERS_WIDTH {
            for y in 0..INVADERS_HIGHT {
                if x > 1 && y > 0 && x % 2 == 0 && y % 2 == 0 {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;

            match self.direction {
                Direction::Left => {
                    let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                    if min_x == 0 {
                        self.direction = Direction::Right;
                        downwards = true;
                    }
                }
                _ => {
                    let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                    if max_x == NUM_COLS - 1 {
                        self.direction = Direction::Left;
                        downwards = true;
                    }
                }
            };

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as isize) + self.direction.get_step_value()) as usize;
                }
            }
            return true;
        }

        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(idx) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y))
        {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let val = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            };
            frame.set_at(invader.x, invader.y, val);
        }
    }
}
