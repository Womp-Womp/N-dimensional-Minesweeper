# Product Requirements Document: N-dimensional Minesweeper

## 1. Vision

To create a mind-bending, visually engaging, and fun puzzle game that extends the classic Minesweeper into N dimensions. The game will be built in Rust, targeting desktop platforms initially, with the potential for future expansion to mobile. It will challenge players' spatial reasoning and provide a unique twist on a well-known game.

## 2. Target Audience

*   **Puzzle Enthusiasts:** Players who enjoy logic puzzles and are looking for a new challenge.
*   **Programmers & Tech Enthusiasts:** Individuals interested in novel applications of technology and computer science concepts.
*   **Students:** Those studying mathematics, physics, or computer science who might appreciate a game that helps visualize higher-dimensional spaces.

## 3. Core Features

### 3.1. Game Logic (The "Model")

*   **N-dimensional Grid:** The game board will be a hyperrectangle of arbitrary dimension `N` and size `(d1, d2, ..., dN)`.
*   **Mine Placement:** A specified number of mines will be randomly distributed throughout the N-dimensional grid after the player's first move.
*   **Cell States:** Each cell in the grid can be in one of the following states:
    *   `Pristine`: Untouched by the player.
    *   `Revealed`: The player has clicked on it.
        *   If it's a mine, the game is over.
        *   If it's not a mine, it displays the number of adjacent mines. Adjacency is defined as any cell that differs by at most 1 in each dimensional coordinate (Moore neighborhood in N dimensions).
    *   `Flagged`: The player has marked it as a potential mine.
*   **Game State:** The game will track win/loss conditions.
    *   **Loss:** A player reveals a mine.
    *   **Win:** All non-mine cells have been revealed.

### 3.2. User Interface & Experience (The "View")

*   **Primary View: 2D Slice Visualization:**
    *   The primary way of interacting with the N-dimensional space will be through a 2D slice.
    *   Players can choose which two dimensions to plot on the X and Y axes of the 2D view.
    *   For dimensions `N > 2`, the remaining `N-2` dimensions must be fixed at specific coordinates. The UI will provide sliders or input boxes for the player to "scroll" through these other dimensions, updating the 2D slice in real-time.
*   **Advanced View (Post-MVP): Angled Slices:** The ability to define a 2D slice at an arbitrary angle through the N-dimensional space. This is a complex feature and should be considered a stretch goal.
*   **Controls:**
    *   Mouse-based interaction for revealing and flagging cells.
    *   UI buttons for starting a new game, selecting difficulty (grid size, dimensions, mine count), and accessing settings.
*   **Information Display:**
    *   A timer.
    *   A counter for remaining mines (total mines minus flagged cells).

### 3.3. Graphics & Sound

*   **Graphics:** Minimalist, clean graphics. Hand-drawn or AI-generated SVGs for numbers, flags, and mines to give the game a unique feel. The focus is on clarity and usability, not graphical intensity.
*   **Sound Effects:** Simple, satisfying sound effects for key actions:
    *   Clicking a cell.
    *   Flagging a cell.
    *   Revealing an empty area.
    *   Game win.
    *   Game over (explosion).

## 4. Technical Stack

*   **Language:** **Rust**. Chosen for its performance, safety, and growing ecosystem.
*   **GUI Framework:** A cross-platform Rust GUI framework is required. Top candidates include:
    *   **`egui`**: An easy-to-use, immediate mode GUI library. Great for rapid development and custom look-and-feel.
    *   **`iced`**: A data-centric, Elm-inspired GUI library. Well-structured and feature-rich.
    *   **`Bevy`**: A full-fledged game engine. Might be overkill, but provides a strong foundation for graphics, state management, and future enhancements.
    *   **`Tauri`**: Allows building a desktop app with a web-based frontend (HTML, CSS, JS). This is a strong contender if we want maximum customizability for the UI and easier portability to mobile later.
    *   **Recommendation:** Start with `egui` or `iced` for the initial desktop MVP due to their simplicity and direct integration with Rust.
*   **Deployment:**
    *   **Desktop:** Create portable executables for Windows, macOS, and Linux.
    *   **Mobile (Future):** Possible via `Tauri` or other tools like `cargo-apk`. This is a post-MVP goal.

## 5. Roadmap & MVP

The project will be developed in phases to ensure a solid foundation and allow for iterative feedback.

### Phase 1: Core Logic & CLI (Command-Line Interface)

*   **Goal:** Implement the core game logic, completely independent of any GUI.
*   **Tasks:**
    1.  Create data structures for the N-dimensional grid.
    2.  Implement mine placement, cell revelation, and flagging logic.
    3.  Implement win/loss conditions.
    4.  Build a simple CLI that allows playing a full game (e.g., a 3x3x3 game) by entering coordinates.
*   **Testing:** Heavy unit testing on all game logic.

### Phase 2: Minimum Viable Product (MVP) - 3D GUI

*   **Goal:** Create the first playable, graphical version of the game.
*   **Tasks:**
    1.  Choose a GUI framework (`egui` is recommended to start).
    2.  Integrate the core game logic from Phase 1.
    3.  Create a 2D slice view for a 3D grid.
    4.  Implement UI controls to scroll through the 3rd dimension (e.g., a slider for the Z-axis).
    5.  Implement mouse controls for revealing/flagging cells.
    6.  Add basic UI elements: "New Game" button, mine counter.
*   **Testing:** Manual testing of the UI with small grids (e.g., 4x4x4).

### Phase 3: Generalize to N-Dimensions

*   **Goal:** Expand the GUI to support arbitrary dimensions.
*   **Tasks:**
    1.  Refactor the UI to handle `N` dimensions.
    2.  Create a settings screen where players can define `N` and the size of each dimension.
    3.  Implement the UI for selecting the two display axes and setting the coordinates for the remaining `N-2` dimensions.
*   **Testing:** Manual testing with 4D and 5D configurations.

### Phase 4: Polish & Extra Features

*   **Goal:** Add features that improve the user experience.
*   **Tasks:**
    1.  Add sound effects.
    2.  Implement custom SVG graphics.
    3.  Add a game timer.
    4.  Improve settings screen (e.g., presets for "Easy," "Medium," "Hard").
    5.  (Stretch Goal) Investigate and implement angled slicing.
    6.  (Stretch Goal) Explore mobile deployment.
