# Vibe Idler: TUI Idle Game Plan

## Context

Build a terminal-based idle game about running a "vibe coding consultancy" where AI agents autonomously build software projects that earn money. The player manages budget, hardware, LLM subscriptions, and agent teams. The TUI should look like a nerdy hacker dashboard with tons of parallel activity — scrolling commit logs, progress bars, sparklines, and color-coded status panels.

**Tech Stack:** Rust + Ratatui (best performance for real-time updates, native sparklines/gauges, cross-platform via Crossterm)

---

## Project Structure

```
vibe-idler/
├── Cargo.toml
├── src/
│   ├── main.rs                   # Terminal setup, panic handler, main loop
│   ├── app.rs                    # App struct, tick + render + input orchestration
│   ├── game/
│   │   ├── mod.rs
│   │   ├── state.rs              # GameState (all saveable data)
│   │   ├── tick.rs               # Per-tick simulation (core engine)
│   │   ├── economy.rs            # Purchase logic, expense/income calc
│   │   ├── formulas.rs           # All math formulas (costs, speed, prestige)
│   │   ├── tech_tree.rs          # Tech tree definitions + unlock conditions
│   │   ├── projects.rs           # Project types, generation, completion
│   │   ├── agents.rs             # Agent behavior, assignment, specialization
│   │   ├── events.rs             # Random event generation
│   │   ├── prestige.rs           # Pivot system (reset + reputation bonuses)
│   │   ├── achievements.rs       # Achievement tracking
│   │   └── offline.rs            # Offline progression calculation
│   ├── ui/
│   │   ├── mod.rs                # Top-level layout, render dispatch
│   │   ├── theme.rs              # Color palette, style constants
│   │   └── panels/
│   │       ├── mod.rs
│   │       ├── header.rs         # Top bar: cash, income/s, compute, agents, rep
│   │       ├── commit_log.rs     # Scrolling fake GitHub commits
│   │       ├── agents.rs         # Agent status panel
│   │       ├── finances.rs       # Income/expense sparklines
│   │       ├── projects.rs       # Project list + progress bars
│   │       ├── shop.rs           # Tech tree / upgrade shop modal
│   │       └── event_log.rs      # Timestamped event feed
│   ├── input.rs                  # Key handling, action mapping
│   ├── save.rs                   # Save/load (JSON, atomic writes)
│   └── data/
│       ├── mod.rs
│       ├── commit_messages.rs    # Fake commit message templates
│       ├── project_names.rs      # Project/client name pools
│       └── event_templates.rs    # Random event text templates
```

**Dependencies:** ratatui 0.30, crossterm 0.28, serde + serde_json, chrono, rand, dirs

---

## Game Mechanics Summary

### Core Loop
- Start: $500 budget. Buy a used laptop ($200), subscribe to basic LLM ($20/mo)
- 1 slow agent grinds small projects (landing pages, scripts) earning $50-200 each
- Reinvest: better hardware, higher LLM tier, more agents, unlock project types
- Scale to 100 agents running enterprise contracts + passive SaaS income

### Resources
- **Cash:** Primary currency. Earned from projects, spent on hardware/LLM/agents
- **Reputation:** Prestige currency. Persists across "Pivots" (resets)
- **Compute:** Sum of hardware power. Affects agent speed (diminishing returns via log2)

### Agents
- Hired with exponentially scaling cost: `100 * 1.5^n`
- Specializations: Generalist, Frontend, Backend, Mobile, DevOps, DataScience, Security, Architect
- Skill grows +0.001/tick while working (caps at 3.0)
- Generate fake commit messages while working

### Projects
- **Contract work:** One-time payment on completion
- **SaaS/Apps:** Recurring monthly income with churn
- **Open Source:** Sponsorship income
- Work required: `base_work * difficulty^1.8`
- Bugs: probability per tick, add 10-30% extra work

### Agent Speed Formula
```
base_speed * llm_quality * log2(1+compute)/5 * spec_bonus * prestige_mult * skill * architect_bonus
```

### Expenses (deducted per game-month = 3000 ticks = 5 real minutes)
- LLM subscription ($0-5000/mo)
- Agent maintenance ($10/agent/mo)
- Hardware maintenance (2% of purchase price/mo)

### Prestige ("Pivot")
- Reputation earned: `floor((lifetime_cash / 10000)^0.5)` — minimum $10k lifetime
- Resets: cash, hardware, agents, projects, unlocks
- Keeps: reputation, prestige perks, achievements
- Bonuses: income mult, speed mult, cost reduction, starting cash, extra agent slots
- Perks at milestones: AutoAssign (10 rep), BugSquasher (25), NegotiationPro (50), PassiveBoost (100), SpeedDemon (200), OfflineGains (500), GoldenTouch (1000)

---

## Tech Tree (4 branches)

**Hardware:** UsedLaptop → RefurbDesktop → GamingPC → Workstation → DualGPU → ServerRack → GPUCluster → DataCenter

**LLM:** FreeTier(0.3) → Basic($20/mo, 1.0) → Pro($50/mo, 1.8) → Team($200/mo, 2.5) → Enterprise($1k/mo, 4.0) → Custom($5k/mo, 6.0)

**Agents:** Slot unlocks (1→2→3→5→10→25→100) + specialization unlocks (Frontend, Backend, Mobile, DevOps, DataSci, Security, Architect)

**Automation:** CI/CD → Testing → CodeReview → ProjectManager → SalesPipeline → ClientRetention → DevEx → SelfImproving

**Projects:** Landing/Personal/Script (free) → CRUD/API → Mobile/Ecommerce → SaaS/Pipeline → ML/Enterprise/OpenSource → Crypto/GameDev

---

## TUI Layout

```
┌─────────────────────────── VIBE IDLER ───────────────────────────┐
│ $ Cash  │  $/s Income  │  Compute  │  Agents  │  Rep: N         │
├──────────────┬───────────────────┬───────────────────────────────┤
│ COMMIT LOG   │ ACTIVE PROJECTS   │ AGENT STATUS                  │
│ (scrolling   │ (progress bars,   │ (name, spec, status,          │
│  fake git    │  ETAs, available  │  skill bar, current           │
│  commits)    │  contracts,       │  project)                     │
│              │  passive income)  │                               │
├──────────────┴───────────────────┴───────────────────────────────┤
│ INCOME ▁▂▃▅▇█  │  EXPENSES ▁▂▃▃▃  │  NET PROFIT ▁▂▃▅▇█         │
├──────────────────────────────────────────────────────────────────┤
│ EVENT LOG (timestamped, color-coded game events)                 │
├──────────────────────────────────────────────────────────────────┤
│ [S]hop  [P]rojects  [A]gents  [T]ech Tree  [V]Pivot  [Q]uit    │
└──────────────────────────────────────────────────────────────────┘
```

**Color theme:** Cyberpunk/hacker — dark blue-black background, neon green for income, red for expenses, cyan for agents, yellow for warnings, purple for prestige.

**Modals:** Shop (tabbed: Hardware/LLM/Agents/Automation), Tech Tree, Pivot screen, Help — all overlay the main dashboard.

---

## Implementation Order

### Sprint 1: Skeleton
1. `cargo init`, add dependencies to Cargo.toml
2. `main.rs`: crossterm terminal setup/teardown, panic handler
3. `app.rs`: App struct with empty tick() and render()
4. `ui/mod.rs`: Render bordered panel layout skeleton
5. `input.rs`: Quit on 'q'
6. **Verify:** Terminal shows bordered panels, quits cleanly

### Sprint 2: Core State + Economy
7. `game/state.rs`: GameState with cash, 1 agent, 1 project
8. `game/formulas.rs`: Cost and speed formulas
9. `game/tick.rs`: Agent works on project, project progresses
10. `ui/panels/header.rs`: Display cash and income/s
11. `ui/panels/projects.rs`: Show project with progress bar
12. **Verify:** Project progresses, cash increases on completion

### Sprint 3: Hardware + LLM + Shop
13. `game/tech_tree.rs`: Hardware/LLM upgrade data
14. `game/economy.rs`: Purchase logic with cost scaling
15. `ui/panels/shop.rs`: Modal shop with Hardware/LLM tabs
16. **Verify:** Buy hardware, change LLM tier, speed increases

### Sprint 4: Agents + Multiple Projects + Commits
17. `game/agents.rs`: Hiring, assignment, specialization
18. `game/projects.rs`: Project generation, multiple active
19. `ui/panels/agents.rs`: Agent status display
20. `ui/panels/commit_log.rs`: Fake commit generation + scrolling
21. `data/commit_messages.rs`: Template pool (~200 combinations)
22. **Verify:** Multiple agents, scrolling commits, multiple projects

### Sprint 5: Passive Income + Events + Sparklines
23. Implement SaaS/app store/sponsorship income
24. `game/events.rs`: Random positive/negative/neutral events
25. `ui/panels/event_log.rs`: Timestamped event display
26. `ui/panels/finances.rs`: Income/expense/net sparklines
27. **Verify:** Passive income flows, events appear, sparklines animate

### Sprint 6: Full Tech Tree + Prestige
28. Complete all tech tree nodes (all 4 branches)
29. Full tech tree view in shop
30. `game/prestige.rs`: Pivot system — reset, reputation, bonuses, perks
31. `game/achievements.rs`: Achievement tracking + display
32. **Verify:** Full progression, pivot works, bonuses apply

### Sprint 7: Save/Load + Offline + Polish
33. `save.rs`: JSON save/load with atomic writes
34. `game/offline.rs`: Offline earnings calculation (50% efficiency, 8hr cap)
35. Auto-save every 60s + on quit
36. ASCII art title banner, final color theme polish
37. Edge cases: terminal resize, going broke, 0 agents
38. **Verify:** Full game loop with persistence, offline progress on reload

---

## Verification Plan
1. `cargo build` — compiles cleanly
2. `cargo run` — launches TUI, shows layout, responds to keys
3. Manual playthrough: start → buy laptop → subscribe LLM → complete projects → hire agents → buy upgrades → reach pivot → pivot → verify bonuses → quit → reload → verify offline earnings
4. Edge cases: resize terminal, let cash hit 0, idle for 10+ minutes, rapid key input
5. `cargo clippy` — no warnings
