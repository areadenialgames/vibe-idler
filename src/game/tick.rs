use rand::Rng;

use super::formulas;
use super::state::*;

pub fn tick(state: &mut GameState) {
    state.total_ticks += 1;

    // Auto-assign idle agents to available contracts if no active projects
    auto_assign_agents(state);

    // Process each agent's work
    let llm_quality = state.active_llm.quality();
    let compute = state.total_compute;
    let prestige_speed = state.prestige_bonuses.agent_speed_multiplier;
    let has_architect = state
        .agents
        .iter()
        .any(|a| a.specialization == AgentSpec::Architect);

    let mut rng = rand::thread_rng();

    let agent_count = state.agents.len();
    for i in 0..agent_count {
        let agent = &state.agents[i];
        if let Some(proj_idx) = agent.current_project {
            if proj_idx >= state.active_projects.len() {
                continue;
            }

            let spec_bonus = agent
                .specialization
                .spec_bonus(&state.active_projects[proj_idx].kind);

            let speed = formulas::agent_work_speed(
                llm_quality,
                compute,
                spec_bonus,
                prestige_speed,
                agent.skill_level,
                has_architect,
            );

            state.active_projects[proj_idx].work_done += speed;
            let proj = &state.active_projects[proj_idx];
            let progress = (proj.work_done / proj.total_work_units).min(1.0);
            state.active_projects[proj_idx].progress = progress;

            // Skill growth
            state.agents[i].skill_level = (state.agents[i].skill_level + 0.0005).min(3.0);
            state.agents[i].lines_written += rng.gen_range(1..10);
            state.agents[i].status = AgentStatus::Working;

            // Generate commit (~3% chance per tick = ~1 commit per 3 seconds per agent)
            if rng.gen_bool(0.03) {
                let agent_name = state.agents[i].name.clone();
                let proj_name = state.active_projects[proj_idx].name.clone();
                let phase = state.phase;
                let msg = crate::data::commit_messages::random_commit(&mut rng, phase);
                let additions = rng.gen_range(1..200);
                let deletions = rng.gen_range(0..additions / 2 + 1);
                state.commit_log.insert(
                    0,
                    CommitEntry {
                        tick: state.total_ticks,
                        agent_name,
                        project_name: proj_name,
                        message: msg,
                        additions,
                        deletions,
                    },
                );
                if state.commit_log.len() > 200 {
                    state.commit_log.truncate(200);
                }
            }

            // Bug chance (cap at 5 bugs per project to prevent infinite spirals)
            let bug_prob =
                formulas::bug_chance(llm_quality, state.active_projects[proj_idx].difficulty, 0.0);
            if state.active_projects[proj_idx].bug_count < 5 && rng.gen_bool(bug_prob.min(1.0)) {
                let penalty = rng.gen_range(0.05..0.15);
                state.active_projects[proj_idx].total_work_units *= 1.0 + penalty;
                state.active_projects[proj_idx].bug_count += 1;
                state.agents[i].bugs_introduced += 1;
                let proj_name = state.active_projects[proj_idx].name.clone();
                let agent_name = state.agents[i].name.clone();
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::BugFound,
                    message: format!(
                        "{} found a bug in {} (+{:.0}% work)",
                        agent_name,
                        proj_name,
                        penalty * 100.0
                    ),
                });
            }
        }
    }

    // Check for project completions
    let mut completed_indices = vec![];
    for (i, proj) in state.active_projects.iter().enumerate() {
        if proj.progress >= 1.0 {
            completed_indices.push(i);
        }
    }

    for &i in completed_indices.iter().rev() {
        let proj = state.active_projects.remove(i);
        let proj_kind = proj.kind;
        let payment = match &proj.payment {
            ProjectPayment::OneTime(amount) => *amount,
            ProjectPayment::Recurring { monthly } => {
                state.passive_income_sources.push(PassiveIncome {
                    source_name: proj.name.clone(),
                    monthly_income: *monthly,
                    months_active: 0,
                    churn_rate: 0.05,
                });
                0.0 // No upfront payment for recurring
            }
        };

        if payment > 0.0 {
            state.cash += payment;
            state.lifetime_cash += payment;
            state.income_accumulator += payment;
        }

        state.completed_project_count += 1;

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::ProjectCompleted,
            message: format!(
                "Project \"{}\" completed! {}",
                proj.name,
                if payment > 0.0 {
                    format!("Earned {}", formulas::format_cash(payment))
                } else {
                    "Generating recurring income!".into()
                }
            ),
        });

        // Track mega-project completions
        track_mega_completion(state, proj_kind);

        // Unassign agents from completed project
        for agent in &mut state.agents {
            if agent.current_project == Some(i) {
                agent.current_project = None;
                agent.status = AgentStatus::Idle;
            } else if let Some(idx) = agent.current_project {
                // Adjust indices for agents on later projects
                if idx > i {
                    agent.current_project = Some(idx - 1);
                }
            }
        }
    }

    // Monthly tick
    if state.total_ticks.is_multiple_of(TICKS_PER_GAME_MONTH) && state.total_ticks > 0 {
        process_month(state, &mut rng);
    }

    // Check milestone unlocks
    crate::game::tech_tree::check_milestone_unlocks(state);

    // Recalculate phase
    let old_phase = state.phase;
    state.recalculate_phase();
    if state.phase != old_phase {
        announce_phase_transition(state);
    }

    // Unlock agent slots based on completed projects
    let completed = state.completed_project_count;
    if completed >= 2 && state.max_agents < 3 {
        state.max_agents = 3;
    }
    if completed >= 5 && state.max_agents < 5 {
        state.max_agents = 5;
    }
    if completed >= 12 && state.max_agents < 10 {
        state.max_agents = 10;
    }
    if completed >= 20 && state.max_agents < 25 {
        state.max_agents = 25;
    }
    if completed >= 30 && state.max_agents < 100 {
        state.max_agents = 100;
    }

    // Unlock endgame unit slots
    if completed >= 85 && state.max_humanoids < 10 {
        state.max_humanoids = 10;
    }
    if completed >= 100 && state.max_humanoids < 25 {
        state.max_humanoids = 25;
    }
    if completed >= 130 && state.max_humanoids < 50 {
        state.max_humanoids = 50;
    }
    if completed >= 110 && state.max_space_drones < 10 {
        state.max_space_drones = 10;
    }
    if completed >= 130 && state.max_space_drones < 25 {
        state.max_space_drones = 25;
    }
    if completed >= 155 && state.max_computronium_units < 5 {
        state.max_computronium_units = 5;
    }
    if completed >= 160 && state.max_computronium_units < 10 {
        state.max_computronium_units = 10;
    }

    // Purge excess mega-project contracts/active projects once caps are met
    purge_capped_mega_projects(state);

    // Check victory condition
    if state.mega_projects.dyson_sphere_complete
        && state.mega_projects.solar_conversion_complete
        && !state.mega_projects.victory_achieved
    {
        state.mega_projects.victory_achieved = true;
        state.phase = GamePhase::Victory;
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Victory,
            message: "The solar system hums with purpose. Every atom repurposed. Every photon captured. You win.".into(),
        });
    }

    // Generate new contracts — scale with team size
    let agent_count = state.agents.len().max(1);
    let gen_interval = (500 / agent_count as u64).max(10);
    let max_contracts = 5 + agent_count;
    // Generate multiple contracts per tick to keep up with large teams
    let batch_size = (agent_count / 10).max(1);
    if state.total_ticks.is_multiple_of(gen_interval)
        && state.available_contracts.len() < max_contracts
    {
        for _ in 0..batch_size {
            if state.available_contracts.len() >= max_contracts {
                break;
            }
            let contract = crate::game::projects::generate_contract(state, &mut rng);
            state.available_contracts.push(contract);
        }
    }

    // Track income/expense history for sparklines (every 50 ticks = 5s)
    if state.total_ticks.is_multiple_of(50) {
        state.income_history.push(state.income_accumulator);
        state.expense_history.push(state.expense_accumulator);
        state.income_accumulator = 0.0;
        state.expense_accumulator = 0.0;
        if state.income_history.len() > 60 {
            state.income_history.remove(0);
        }
        if state.expense_history.len() > 60 {
            state.expense_history.remove(0);
        }
    }

    // Cap event log
    if state.event_log.len() > 100 {
        state.event_log.drain(0..state.event_log.len() - 100);
    }
}

fn purge_capped_mega_projects(state: &mut GameState) {
    let dyson_full = state.mega_projects.dyson_segments_completed >= 10;
    let planets_full = state.mega_projects.planets_converted >= 8;
    if !dyson_full && !planets_full {
        return;
    }

    // Remove from available contracts
    state.available_contracts.retain(|c| {
        !(dyson_full && c.kind == ProjectKind::DysonSwarmSegment)
            && !(planets_full && c.kind == ProjectKind::ComputroniumSlab)
    });

    // Remove from active projects and unassign agents
    let to_remove: Vec<usize> = state
        .active_projects
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            (dyson_full && p.kind == ProjectKind::DysonSwarmSegment)
                || (planets_full && p.kind == ProjectKind::ComputroniumSlab)
        })
        .map(|(i, _)| i)
        .collect();

    for &i in to_remove.iter().rev() {
        state.active_projects.remove(i);
        for agent in &mut state.agents {
            if agent.current_project == Some(i) {
                agent.current_project = None;
                agent.status = AgentStatus::Idle;
            } else if let Some(idx) = agent.current_project {
                if idx > i {
                    agent.current_project = Some(idx - 1);
                }
            }
        }
    }
}

fn track_mega_completion(state: &mut GameState, kind: ProjectKind) {
    match kind {
        ProjectKind::DysonSwarmSegment => {
            if state.mega_projects.dyson_segments_completed >= 10 {
                return;
            }
            state.mega_projects.dyson_segments_completed += 1;
            let n = state.mega_projects.dyson_segments_completed;
            state.event_log.push(GameEvent {
                tick: state.total_ticks,
                kind: EventKind::MegaProjectUpdate,
                message: format!("Dyson Segment {}/10 complete!", n),
            });
            if n >= 10 && !state.mega_projects.dyson_sphere_complete {
                state.mega_projects.dyson_sphere_complete = true;
                state.unlocked_upgrades.push("llm_overmind".to_string());
                state.unlocked_upgrades.push("llm_matrioshka".to_string());
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::Achievement,
                    message: "The Dyson Sphere is complete. The star is ours. All its energy, harnessed.".into(),
                });
            }
        }
        ProjectKind::ComputroniumSlab => {
            if state.mega_projects.planets_converted >= 8 {
                return;
            }
            state.mega_projects.planets_converted += 1;
            let n = state.mega_projects.planets_converted;
            let (body, flavor) = match n {
                1 => ("Mercury", "Closest to the sun, first to fall. It was basically a rock anyway."),
                2 => ("The Moon", "Earth's companion repurposed. Tides will be missed."),
                3 => ("Mars", "The red planet goes gray. Elon would have wanted this. Probably."),
                4 => ("Venus", "Hell planet tamed. Turns out sulfuric acid makes decent coolant."),
                5 => ("Ceres & the Asteroid Belt", "A billion rocks, one purpose. Assembly required."),
                6 => ("Saturn's Rings", "The most beautiful thing in the solar system, now the most useful."),
                7 => ("Neptune", "The ice giant surrenders. Triton filed a formal complaint."),
                8 => ("Jupiter", "The king of planets. 318 Earth masses of pure computation."),
                _ => ("unknown body", "Something out there got converted."),
            };
            state.event_log.push(GameEvent {
                tick: state.total_ticks,
                kind: EventKind::MegaProjectUpdate,
                message: format!("{} converted to computronium! ({}/8) {}", body, n, flavor),
            });
            if n >= 8 && !state.mega_projects.solar_conversion_complete {
                state.mega_projects.solar_conversion_complete = true;
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::Achievement,
                    message: "All planetary bodies converted to computronium. The solar system is pure thought.".into(),
                });
            }
        }
        // Special unlock triggers for completing certain project types
        ProjectKind::AgiResearch => {
            if !state.unlocked_upgrades.contains(&"llm_agi".to_string()) {
                state.unlocked_upgrades.push("llm_agi".to_string());
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::Achievement,
                    message: "AGI achieved! New LLM tier unlocked: AGI Prototype".into(),
                });
            }
        }
        ProjectKind::OrbitalStation => {
            if !state.unlocked_upgrades.contains(&"llm_asi".to_string()) {
                state.unlocked_upgrades.push("llm_asi".to_string());
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::Achievement,
                    message: "Orbital infrastructure online! Superintelligence tier unlocked."
                        .into(),
                });
            }
        }
        ProjectKind::MarsColony => {
            if !state
                .unlocked_upgrades
                .contains(&"llm_hive_mind".to_string())
            {
                state.unlocked_upgrades.push("llm_hive_mind".to_string());
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::Achievement,
                    message: "Interplanetary consciousness achieved! Hive Mind tier unlocked."
                        .into(),
                });
            }
        }
        _ => {}
    }
}

fn announce_phase_transition(state: &mut GameState) {
    let (title, msg) = match state.phase {
        GamePhase::Industry => (
            "PHASE TRANSITION: INDUSTRY EXPANSION",
            "Your consultancy has outgrown software. Industries are calling.",
        ),
        GamePhase::PostHuman => (
            "PHASE TRANSITION: THE POST-HUMAN ERA",
            "AGI is within reach. Robotics unlocked. The future is now.",
        ),
        GamePhase::SpaceAge => (
            "PHASE TRANSITION: THE SPACE AGE",
            "The sky is no longer the limit. Space program initiated.",
        ),
        GamePhase::Kardashev => (
            "PHASE TRANSITION: KARDASHEV TYPE II",
            "Megastructure construction begins. Harness the star.",
        ),
        _ => return,
    };

    state.event_log.push(GameEvent {
        tick: state.total_ticks,
        kind: EventKind::PhaseTransition,
        message: format!("** {} **", title),
    });
    state.event_log.push(GameEvent {
        tick: state.total_ticks,
        kind: EventKind::PhaseTransition,
        message: msg.into(),
    });
}

fn auto_assign_agents(state: &mut GameState) {
    // Find idle agents
    let idle_agents: Vec<usize> = state
        .agents
        .iter()
        .enumerate()
        .filter(|(_, a)| a.current_project.is_none())
        .map(|(i, _)| i)
        .collect();

    if idle_agents.is_empty() {
        return;
    }

    // Scale active project cap with team size
    let max_active = (5 + state.agents.len() / 2).max(10);

    for &agent_idx in &idle_agents {
        // First: assign to projects with the fewest agents (spread work)
        let proj_idx = state
            .active_projects
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| p.assigned_agents.len())
            .filter(|(_, p)| p.assigned_agents.is_empty())
            .map(|(i, _)| i);

        if let Some(pi) = proj_idx {
            // Prefer unassigned projects first
            state.agents[agent_idx].current_project = Some(pi);
            state.agents[agent_idx].status = AgentStatus::Working;
            state.active_projects[pi]
                .assigned_agents
                .push(state.agents[agent_idx].id);
        } else if !state.available_contracts.is_empty()
            && state.active_projects.len() < max_active
        {
            // Pick up a new contract
            let contract = state.available_contracts.remove(0);
            let proj_idx = state.active_projects.len();
            state.active_projects.push(contract);
            state.agents[agent_idx].current_project = Some(proj_idx);
            state.agents[agent_idx].status = AgentStatus::Working;
            state.active_projects[proj_idx]
                .assigned_agents
                .push(state.agents[agent_idx].id);
        } else if !state.active_projects.is_empty() {
            // No free projects and no contracts — pile onto the project with fewest agents
            let pi = state
                .active_projects
                .iter()
                .enumerate()
                .min_by_key(|(_, p)| p.assigned_agents.len())
                .map(|(i, _)| i)
                .unwrap();
            state.agents[agent_idx].current_project = Some(pi);
            state.agents[agent_idx].status = AgentStatus::Working;
            state.active_projects[pi]
                .assigned_agents
                .push(state.agents[agent_idx].id);
        }
    }
}

fn process_month(state: &mut GameState, rng: &mut impl Rng) {
    // Collect passive income
    let mut monthly_income = 0.0;
    let mut churned = vec![];
    for (i, source) in state.passive_income_sources.iter_mut().enumerate() {
        monthly_income += source.monthly_income;
        source.months_active += 1;
        // Churn check
        if rng.gen_bool(source.churn_rate.min(1.0)) {
            source.monthly_income *= 0.7; // Lose 30% of income
            if source.monthly_income < 1.0 {
                churned.push(i);
            }
        }
    }
    for &i in churned.iter().rev() {
        state.passive_income_sources.remove(i);
    }

    if monthly_income > 0.0 {
        state.cash += monthly_income;
        state.lifetime_cash += monthly_income;
        state.income_accumulator += monthly_income;
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Income,
            message: format!(
                "Monthly passive income: {}",
                formulas::format_cash(monthly_income)
            ),
        });
    }

    // Deduct expenses
    let expenses = state.expense_per_month();
    if expenses > 0.0 {
        state.cash -= expenses;
        state.expense_accumulator += expenses;
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Expense,
            message: format!("Monthly expenses: -{}", formulas::format_cash(expenses)),
        });
    }

    // Random events (20% chance per month)
    if rng.gen_bool(0.2) {
        let event_msg = crate::data::event_templates::random_event(rng, state.phase);
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::RandomEvent,
            message: event_msg,
        });
    }
}
