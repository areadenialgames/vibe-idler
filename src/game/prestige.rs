use super::formulas;
use super::state::*;

pub fn calculate_pivot_reputation(state: &GameState) -> f64 {
    formulas::prestige_reputation_earned(state.lifetime_cash)
}

pub fn perform_pivot(state: &mut GameState) {
    let new_rep = calculate_pivot_reputation(state);
    if new_rep <= 0.0 {
        return;
    }

    state.reputation += new_rep;
    state.pivot_count += 1;

    // Recalculate bonuses
    let rep = state.reputation;
    state.prestige_bonuses.income_multiplier = 1.0 + rep.sqrt() * 0.10;
    state.prestige_bonuses.agent_speed_multiplier = 1.0 + rep.sqrt() * 0.05;
    state.prestige_bonuses.cost_reduction = (rep.sqrt() * 0.02).min(0.50);
    state.prestige_bonuses.starting_cash_bonus = 500.0 + rep * 50.0;
    state.prestige_bonuses.extra_agent_slots = rep.sqrt().floor() as u32;

    // Preserve audio preferences across pivots
    let audio_enabled = state.audio_enabled;
    let radio_enabled = state.radio_enabled;
    let radio_station = state.radio_station;

    // Reset game state (keep prestige)
    let starting_cash = 500.0 + state.prestige_bonuses.starting_cash_bonus;
    state.cash = starting_cash;
    state.lifetime_cash = 0.0;
    state.total_ticks = 0;
    state.hardware.clear();
    state.total_compute = 0.0;
    state.active_llm = LlmTier::FreeTier;
    state.agents.clear();
    state.agents.push(Agent {
        id: 0,
        name: "Agent-1".into(),
        specialization: AgentSpec::Generalist,
        skill_level: 1.0,
        status: AgentStatus::Idle,
        current_project: None,
        lines_written: 0,
        bugs_introduced: 0,
    });
    state.max_agents = 2;
    state.active_projects.clear();
    state.completed_project_count = 0;
    state.available_contracts.clear();
    state.passive_income_sources.clear();
    state.unlocked_upgrades = vec![
        "hw_used_laptop".into(),
        "llm_free".into(),
        "proj_landing".into(),
        "proj_personal_site".into(),
        "proj_simple_script".into(),
    ];
    state.commit_log.clear();
    state.event_log.clear();
    state.income_history = vec![0.0; 60];
    state.expense_history = vec![0.0; 60];

    // Restore audio preferences
    state.audio_enabled = audio_enabled;
    state.radio_enabled = radio_enabled;
    state.radio_station = radio_station;

    state.event_log.push(GameEvent {
        tick: 0,
        kind: EventKind::Achievement,
        message: format!(
            "PIVOT #{} complete! Earned {} reputation. Bonuses active!",
            state.pivot_count, new_rep as u64
        ),
    });
}
