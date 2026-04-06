use std::time::Duration;

use crate::{
    NUM_COLS, NUM_ROWS, PLAYER_SHOTS_MAX_COUNT, frame::Frame, invaders::Invaders, shot::Shot,
    traits::Drawable,
};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            if self.x < NUM_COLS - 1 {
                self.x += 1;
            }
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shots.len() < PLAYER_SHOTS_MAX_COUNT
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < PLAYER_SHOTS_MAX_COUNT {
            self.shots.push(Shot::new(self.x, self.y - 1));
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.is_dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame.set_at(self.x, self.y, "A");
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
