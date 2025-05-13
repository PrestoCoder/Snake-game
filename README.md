# 🐍 Terminal Snake Game in Rust

A feature-rich terminal-based implementation of the classic Snake game in Rust — complete with multiple levels, dynamic obstacles, and progressive difficulty.

---

## 🚀 Features

- 🐍 Classic snake mechanics with modern enhancements
- 🎮 Smooth terminal UI powered by `crossterm`
- 🏁 Multiple levels with escalating difficulty
- 🧱 **Dynamic obstacle patterns that grow more complex with each level**
- 📈 Speed increases as you eat more food
- 💯 Score tracking and level-based progression
- 🎯 Intelligent and fair food placement
- 🚧 Collision detection (walls, obstacles, self)

---

## 🆕 What's New Compared to Standard Snake?

- 🧱 **Obstacles**: Each level introduces new obstacle patterns
- 📈 **Progressive Challenge**: Difficulty increases through obstacle complexity and snake speed
- 🗺️ **Level-Based Design**: Structured advancement adds long-term goals and variety

---

## 📁 Project Structure

```plaintext
src/
├── core/             # Core game mechanics
│   ├── collision.rs     # Collision detection logic
│   ├── scoring.rs       # Score management
│   └── state.rs         # Game state transitions
│
├── entities/         # Game objects
│   ├── direction.rs     # Direction and movement
│   ├── food.rs          # Food generation logic
│   ├── obstacle.rs      # Obstacle definitions
│   └── point.rs         # 2D point abstraction
│
├── gameplay/         # Gameplay systems
│   ├── level_state.rs    # Level transitions
│   ├── pattern_generator.rs # Obstacle pattern generator
│   └── snake.rs          # Snake behavior and logic
│
├── ui/               # User interface handling
│   ├── display.rs       # Text-based UI output
│   ├── input.rs         # Input listener
│   └── renderer.rs      # Rendering engine
│
├── utils/            # Utility modules
│   ├── constants.rs     # Game constants
│   └── error.rs         # Custom error types
│
├── config.rs         # Configurable game parameters
├── lib.rs            # Library entry point
└── main.rs           # Application entry point
```

---

## 🧰 Requirements

- Rust 1.70 or higher (tested with rustc 1.86.0)
- Terminal with ANSI support
- Minimum terminal size: `50x25` characters

---

## 🛠️ Installation

```bash
git clone https://github.com/YOUR_USERNAME/snake-game.git
cd snake-game
cargo build --release
cargo run --release
```

---

## 🎮 How to Play

### Controls

- `↑` or `W` – Move Up
- `↓` or `S` – Move Down
- `←` or `A` – Move Left
- `→` or `D` – Move Right
- `SPACE` – Advance to next level
- `Q` – Quit the game

### Game Rules

- Eat food (`●`) to grow and score points
- Avoid:

  - Walls (`█` in blue)
  - Obstacles (`█` in grey)
  - Your own body

- Score enough to progress to the next level
- Snake speed increases with each food item
- Obstacle layouts become more complex in each level
- Beat all levels to win!

---

## 🧪 Development & Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test scoring_tests

# Show test output
cargo test -- --nocapture
```

### Test Coverage

- Unit tests for core modules
- Integration tests for gameplay
- Tests for:

  - Collision detection
  - Food generation
  - Score tracking
  - Level progression

---

## ⚙️ Configuration

Customize game settings in [`src/config.rs`](src/config.rs):

- Game board dimensions
- Initial speed and difficulty
- Level-specific configurations
- Obstacle layouts
- Snake start position

---

## 🧠 Technical Highlights

- **State Manager**: Manages game state transitions
- **Collision System**: Efficient and extensible design
- **Double-buffered Renderer**: Flicker-free terminal drawing
- **Input Handling**: Non-blocking and responsive controls
- **Pattern Generator**: Procedural obstacle layouts

### Performance Optimizations

- HashSet-based collision detection
- Minimal screen redraws per tick
- Configurable tick rate for performance tuning

---
