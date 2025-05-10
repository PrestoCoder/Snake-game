use snake_game::{
    engine::{Renderer, InputHandler},
    game::GameState,
    config::Config,
    error::Result,
};
use std::{
    time::{Duration, Instant},
    thread,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    Result as CrosstermResult,
};

fn main() -> CrosstermResult<()> {
    // Enable raw mode
    enable_raw_mode()?;

    // Run the game
    if let Err(e) = run_game() {
        eprintln!("Error: {:?}", e);
    }

    // Ensure we disable raw mode
    disable_raw_mode()?;
    Ok(())
}

fn run_game() -> Result<()> {
    // Load configuration
    let config = Config::load()?;
    
    // Initialize game components
    let mut game_state = GameState::new(config.width, config.height);
    let mut renderer = Renderer::new(config.width, config.height);
    let input_handler = InputHandler::new();

    // Initialize terminal
    renderer.init()?;

    // Give the terminal a moment to initialize
    thread::sleep(Duration::from_millis(100));

    // Initial render
    renderer.render(&game_state)?;

    // Game loop
    let tick_rate = Duration::from_millis(100); // Slower initial speed
    let frame_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();
    let mut last_render = Instant::now();

    while !game_state.is_game_over() {
        // Handle input with a small timeout
        if let Some(direction) = input_handler.get_input()? {
            game_state.change_direction(direction);
        }

        // Update game state at tick rate
        if last_tick.elapsed() >= tick_rate {
            game_state.update()?;
            last_tick = Instant::now();
        }

        // Render at frame rate
        if last_render.elapsed() >= frame_rate {
            renderer.render(&game_state)?;
            last_render = Instant::now();
        }

        // Small sleep to prevent CPU hogging
        thread::sleep(Duration::from_millis(10));
    }

    // Show game over screen and wait for input
    renderer.render(&game_state)?;
    thread::sleep(Duration::from_secs(1));

    // Wait for any key press before exiting
    while let Ok(None) = input_handler.get_input() {
        thread::sleep(Duration::from_millis(100));
    }

    renderer.cleanup()?;
    Ok(())
}