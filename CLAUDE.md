# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run

```sh
cargo build              # debug build
cargo build --release     # release build
cargo run                 # run in debug mode
cargo run --release       # run in release mode
cargo clippy              # lint
cargo fmt                 # format
```

No tests exist yet. No CI configuration.

## Architecture

Rust TUI idle game built with Ratatui + Crossterm. The player manages a "vibe coding consultancy" — hiring AI agents, buying hardware, and completing software contracts.

### Core Loop

`main.rs` sets up a 100ms tick loop:
1. Render UI via `ui::render()`
2. Poll keyboard input → `app.handle_input()` dispatches `Action` enum
3. Call `app.tick()` → `game::tick::tick(&mut state)` for all game logic
4. Auto-save every 60 seconds

### Module Responsibilities

- **`app.rs`** — `App` struct holds `GameState` + `UiState` (modal, tab, selection). Routes input actions to game state mutations.
- **`input.rs`** — Maps `KeyEvent` → `Action` enum. Context-aware: different mappings when a modal is open vs main screen.
- **`save.rs`** — JSON save/load via serde. Uses atomic write (tmp file + rename). Platform-aware paths via `dirs` crate.

- **`game/state.rs`** — `GameState` is the single source of truth. All game data: cash, agents, projects, hardware, LLM tier, unlocks, prestige bonuses, event/commit logs. All enums live here (`HardwareKind`, `LlmTier`, `AgentSpec`, `ProjectKind`).
- **`game/tick.rs`** — Per-tick logic: agent work calculation, bug generation, commit creation, project completion, monthly income/expenses (every 3000 ticks), contract generation, tech tree checks.
- **`game/formulas.rs`** — All math: cost scaling, work speed, bug chance, prestige reputation. Key formula: `work_speed = 0.1 * llm_quality * log2(1+compute)/5 * prestige_speed * skill * spec_bonus * arch_bonus`.
- **`game/economy.rs`** — Purchase dispatch: `try_purchase(state, tab, item_idx)` → buy hardware/LLM/agents.
- **`game/projects.rs`** — Contract generation with random names, difficulty, payment. SaaS/OpenSource create recurring `PassiveIncome`.
- **`game/prestige.rs`** — Pivot resets game state but keeps reputation-based multipliers (income, speed, cost reduction, starting cash, extra agent slots).
- **`game/tech_tree.rs`** — Milestone unlocks keyed on hardware tier ownership and `completed_project_count`.

- **`ui/mod.rs`** — Main `render()` splits terminal into 5 vertical areas: header, main content (3 columns), finances sparklines, event log, hotkey bar. Handles modal overlays (Shop, Help).
- **`ui/theme.rs`** — Color palette constants.
- **`ui/panels/`** — One file per panel: `header.rs`, `projects.rs`, `agents.rs`, `commit_log.rs`, `event_log.rs`, `finances.rs`, `shop.rs`.

- **`data/`** — Procedural content generators for commit messages, project names, and event flavor text.

### Key Patterns

- **Tick-driven**: All game logic runs through `game::tick::tick()`. No async, no threads.
- **Modal input routing**: `UiState.modal` (None/Shop/Help) determines which key mappings are active.
- **Exponential cost scaling**: Hardware and agent costs use `base * growth_rate^owned` curves, reduced by prestige bonuses.
- **String-based unlocks**: `GameState.unlocked_upgrades: Vec<String>` tracks what's available; checked before rendering shop items.
- **Auto-assignment**: Idle agents automatically claim available contracts each tick.
