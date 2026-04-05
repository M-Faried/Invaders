use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use invaders::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    player::Player,
    screen::Screen,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io, thread,
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

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut screen = Screen::new();
    screen.start();

    // Game Loop
    let mut instant = Instant::now();
    let mut player: Player = Player::new();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // calculating detla
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = Frame::new();

        // reading key presses
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.can_shoot() {
                            player.shoot();
                            audio.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        // updating the frame
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // drawing the frame
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);

        // sending the frame to the render thread
        screen.update_with_frame(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // win or lose
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
    screen.stop();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
