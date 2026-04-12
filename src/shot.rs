use std::time::Duration;

use rusty_time::timer::Timer;

use crate::{
    SHOT_EXPLODING_INTERVAL, SHOT_UPDATE_INTERVAL,
    frame::Frame,
    traits::{Drawable, Tickable},
};

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

impl Tickable for Shot {
    fn tick(&mut self, delta: Duration) -> bool {
        let mut changed = false;
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
                changed = true;
            }
            self.timer.reset();
        }
        changed
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        let val = if self.exploding { "*" } else { "|" };
        frame.set_at(self.x, self.y, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shot_creation() {
        let shot = Shot::new(5, 10);
        assert_eq!(shot.x, 5);
        assert_eq!(shot.y, 10);
        assert!(!shot.exploding);
        assert!(!shot.is_dead());
    }

    #[test]
    fn test_shot_movement() {
        let mut shot = Shot::new(5, 10);
        shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
        assert_eq!(shot.y, 9);
        assert!(!shot.exploding);
    }

    #[test]
    fn test_shot_multiple_movements() {
        let mut shot = Shot::new(5, 10);
        for expected_y in (1..10).rev() {
            shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
            assert_eq!(shot.y, expected_y);
        }
    }

    #[test]
    fn test_shot_reaches_top() {
        let mut shot = Shot::new(5, 1);
        shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
        assert_eq!(shot.y, 0);
        assert!(shot.is_dead());
    }

    #[test]
    fn test_shot_doesnt_move_below_zero() {
        let mut shot = Shot::new(5, 0);
        shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
        assert_eq!(shot.y, 0);
    }

    #[test]
    fn test_shot_explode() {
        let mut shot = Shot::new(5, 5);
        assert!(!shot.exploding);
        shot.explode();
        assert!(shot.exploding);
    }

    #[test]
    fn test_shot_exploding_timer() {
        let mut shot = Shot::new(5, 5);
        shot.explode();
        assert!(!shot.is_dead());
        shot.tick(Duration::from_millis(SHOT_EXPLODING_INTERVAL as u64 + 1));
        assert!(shot.is_dead());
    }

    #[test]
    fn test_shot_stops_moving_when_exploding() {
        let mut shot = Shot::new(5, 10);
        shot.explode();
        let y_before = shot.y;
        shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
        assert_eq!(shot.y, y_before);
    }

    #[test]
    fn test_shot_draw_normal() {
        let shot = Shot::new(3, 5);
        let mut frame = Frame::new();
        shot.draw(&mut frame);
        assert_eq!(frame.get_at(3, 5), "|");
    }

    #[test]
    fn test_shot_draw_exploding() {
        let mut shot = Shot::new(3, 5);
        shot.explode();
        let mut frame = Frame::new();
        shot.draw(&mut frame);
        assert_eq!(frame.get_at(3, 5), "*");
    }

    #[test]
    fn test_shot_x_coordinate_preserved() {
        let mut shot = Shot::new(7, 10);
        for _ in 0..5 {
            shot.tick(Duration::from_millis(SHOT_UPDATE_INTERVAL as u64 + 1));
        }
        assert_eq!(shot.x, 7);
    }

    #[test]
    fn test_shot_update_without_ready_timer() {
        let mut shot = Shot::new(5, 10);
        let y_before = shot.y;
        shot.tick(Duration::from_millis(1));
        assert_eq!(shot.y, y_before);
    }
}
