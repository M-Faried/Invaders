use invaders::{
    FRAME_REFRESH_INTERVAL,
    display::Display,
    frame::Frame,
    invaders::Invaders,
    keyboard::{GameCommand, get_kb_command},
    player::Player,
    traits::Drawable,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/explode.wav");
    audio.add("lose", "sounds/lose.wav");
    audio.add("move", "sounds/move.wav");
    audio.add("pew", "sounds/pew.wav");
    audio.add("startup", "sounds/startup.wav");
    audio.add("win", "sounds/win.wav");
    audio.play("startup");

    let mut display = Display::new();
    let mut instant = Instant::now();
    let mut player = Player::new();
    let mut invaders = Invaders::new();

    display.init()?;

    // Game Loop
    'gameloop: loop {
        // calculating detla of the time
        let delta = instant.elapsed();
        instant = Instant::now();

        match get_kb_command() {
            GameCommand::MoveLeft => player.move_left(),
            GameCommand::MoveRight => player.move_right(),
            GameCommand::Shoot => {
                if player.can_shoot() {
                    player.shoot();
                    audio.play("pew");
                }
            }
            GameCommand::Exit => {
                audio.play("lose");
                break 'gameloop;
            }
            _ => {}
        }

        // conditional sound effects
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // the new frame to be displayed
        let mut curr_frame = Frame::new();

        // updating the frame
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);

        // updating screen
        display.update(curr_frame);

        thread::sleep(Duration::from_millis(FRAME_REFRESH_INTERVAL));

        // win or lose check
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup section
    audio.wait();
    display.clear()?;
    Ok(())
}
