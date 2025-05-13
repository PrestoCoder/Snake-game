use snake_game::{
    ui::{Renderer, InputHandler},
    core::GameState,
    config::Config,
    utils::Result,
    entities::Direction,
    gameplay::GameState as GameStateEnum,
};
use std::{
    time::{Duration, Instant},
    thread,
    fs::OpenOptions,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::KeyCode,
    Result as CrosstermResult,
};
use env_logger::{Builder, Target};

fn main() -> CrosstermResult<()> {
    // Initialize logging to file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("snake_game.log")
        .unwrap();

    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .filter_level(log::LevelFilter::Debug)
        .init();

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
    let mut game_state = GameState::new(&config);
    let mut renderer = Renderer::new(config.width, config.height);
    let input_handler = InputHandler::new();

    // Initialize terminal
    renderer.init()?;

    // Give the terminal a moment to initialize
    thread::sleep(Duration::from_millis(100));

    // Initial render
    renderer.render(&game_state)?;

    // Game loop timing
    let frame_rate = Duration::from_millis(50); // ~30 FPS
    let mut last_tick = Instant::now();
    let mut last_render = Instant::now();

    while !matches!(game_state.game_state(), GameStateEnum::GameOver(_)) {
        // Handle input
        if let Ok(Some(key)) = input_handler.get_input() {
            match game_state.game_state() {
                GameStateEnum::Playing => {
                    match key {
                        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                            game_state.change_direction(Direction::Up);
                        }
                        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                            game_state.change_direction(Direction::Down);
                        }
                        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                            game_state.change_direction(Direction::Left);
                        }
                        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                            game_state.change_direction(Direction::Right);
                        }
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
                GameStateEnum::LevelTransition => {
                    match key {
                        KeyCode::Char(' ') => game_state.start_next_level(),
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
                GameStateEnum::GameOver(_) => {
                    if matches!(key, KeyCode::Char('q') | KeyCode::Char('Q')) {
                        break;
                    }
                }
            }
        }

        match game_state.game_state() {
            GameStateEnum::Playing => {
                // Get current tick rate based on speed level
                let current_tick_rate = Duration::from_millis(game_state.get_tick_rate());

                // Update game state at current speed
                if last_tick.elapsed() >= current_tick_rate {
                    game_state.update()?;
                    last_tick = Instant::now();
                }
            }
            _ => {
                // For transitions and game over, just update without timing
                game_state.update()?;
            }
        }

        // Render at frame rate
        if last_render.elapsed() >= frame_rate {
            renderer.render(&game_state)?;
            last_render = Instant::now();
        }

        // Small sleep to prevent CPU hogging
        thread::sleep(Duration::from_millis(16));
    }

    // Show final state
    renderer.render(&game_state)?;
    
    // Wait for a moment before exit
    thread::sleep(Duration::from_secs(1));

    // Cleanup
    renderer.cleanup()?;
    Ok(())
}