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

    let mut rng = rand::thread_rng();

    let agent_count = state.agents.len();
    for i in 0..agent_count {
        let agent = &state.agents[i];
        if let Some(proj_idx) = agent.current_project {
            if proj_idx >= state.active_projects.len() {
                continue;
            }

            let speed = formulas::agent_work_speed(
                llm_quality,
                compute,
                false, // TODO: spec matching
                prestige_speed,
                agent.skill_level,
                false, // TODO: architect check
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
                let msg = crate::data::commit_messages::random_commit(&mut rng);
                let additions = rng.gen_range(1..200);
                let deletions = rng.gen_range(0..additions / 2 + 1);
                state.commit_log.insert(0, CommitEntry {
                    tick: state.total_ticks,
                    agent_name,
                    project_name: proj_name,
                    message: msg,
                    additions,
                    deletions,
                });
                if state.commit_log.len() > 200 {
                    state.commit_log.truncate(200);
                }
            }

            // Bug chance
            let bug_prob = formulas::bug_chance(llm_quality, state.active_projects[proj_idx].difficulty, 0.0);
            if rng.gen_bool(bug_prob.min(1.0)) {
                let penalty = rng.gen_range(0.10..0.30);
                state.active_projects[proj_idx].total_work_units *= 1.0 + penalty;
                state.active_projects[proj_idx].bug_count += 1;
                state.agents[i].bugs_introduced += 1;
                let proj_name = state.active_projects[proj_idx].name.clone();
                let agent_name = state.agents[i].name.clone();
                state.event_log.push(GameEvent {
                    tick: state.total_ticks,
                    kind: EventKind::BugFound,
                    message: format!("{} found a bug in {} (+{:.0}% work)", agent_name, proj_name, penalty * 100.0),
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
    if state.total_ticks % TICKS_PER_GAME_MONTH == 0 && state.total_ticks > 0 {
        process_month(state, &mut rng);
    }

    // Check milestone unlocks
    crate::game::tech_tree::check_milestone_unlocks(state);

    // Generate new contracts periodically
    if state.total_ticks % 500 == 0 && state.available_contracts.len() < 5 {
        let contract = crate::game::projects::generate_contract(state, &mut rng);
        state.available_contracts.push(contract);
    }

    // Track income/expense history for sparklines (every 50 ticks = 5s)
    if state.total_ticks % 50 == 0 {
        let income = state.income_per_tick() * 50.0;
        let expense = state.expense_per_month() / TICKS_PER_GAME_MONTH as f64 * 50.0;
        state.income_history.push(income);
        state.expense_history.push(expense);
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

fn auto_assign_agents(state: &mut GameState) {
    // Find idle agents
    let idle_agents: Vec<usize> = state.agents.iter().enumerate()
        .filter(|(_, a)| a.current_project.is_none())
        .map(|(i, _)| i)
        .collect();

    if idle_agents.is_empty() {
        return;
    }

    // If there are active projects that need agents, assign there
    for &agent_idx in &idle_agents {
        // Find a project with no agents assigned
        let proj_idx = state.active_projects.iter().enumerate()
            .find(|(_, p)| p.assigned_agents.is_empty())
            .map(|(i, _)| i);

        if let Some(pi) = proj_idx {
            state.agents[agent_idx].current_project = Some(pi);
            state.agents[agent_idx].status = AgentStatus::Working;
            state.active_projects[pi].assigned_agents.push(state.agents[agent_idx].id);
        } else if !state.available_contracts.is_empty() && state.active_projects.len() < 10 {
            // Accept a contract automatically if agents are idle with nothing to do
            let contract = state.available_contracts.remove(0);
            let proj_idx = state.active_projects.len();
            state.active_projects.push(contract);
            state.agents[agent_idx].current_project = Some(proj_idx);
            state.agents[agent_idx].status = AgentStatus::Working;
            state.active_projects[proj_idx].assigned_agents.push(state.agents[agent_idx].id);
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
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Income,
            message: format!("Monthly passive income: {}", formulas::format_cash(monthly_income)),
        });
    }

    // Deduct expenses
    let expenses = state.expense_per_month();
    if expenses > 0.0 {
        state.cash -= expenses;
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Expense,
            message: format!("Monthly expenses: -{}", formulas::format_cash(expenses)),
        });
    }

    // Random events (20% chance per month)
    if rng.gen_bool(0.2) {
        let event_msg = crate::data::event_templates::random_event(rng);
        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::RandomEvent,
            message: event_msg,
        });
    }
}
