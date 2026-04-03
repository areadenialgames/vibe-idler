use super::formulas;
use super::state::*;

pub fn try_purchase(state: &mut GameState, tab: usize, item_idx: usize) {
    match tab {
        0 => buy_hardware(state, item_idx),
        1 => buy_llm(state, item_idx),
        2 => hire_agent(state),
        3 => {} // Automation - Sprint 6
        _ => {}
    }
}

fn buy_hardware(state: &mut GameState, item_idx: usize) {
    let all = HardwareKind::all();
    if item_idx >= all.len() {
        return;
    }
    let kind = all[item_idx];

    // Check unlock
    if !state.unlocked_upgrades.contains(&kind.unlock_id().to_string()) {
        return;
    }

    let owned = state.hardware.iter()
        .find(|h| h.kind == kind)
        .map(|h| h.count)
        .unwrap_or(0);

    let cost = formulas::hardware_cost(
        kind.base_cost(),
        kind.growth_rate(),
        owned,
        state.prestige_bonuses.cost_reduction,
    );

    if state.cash >= cost {
        state.cash -= cost;

        if let Some(hw) = state.hardware.iter_mut().find(|h| h.kind == kind) {
            hw.count += 1;
        } else {
            state.hardware.push(OwnedHardware { kind, count: 1 });
        }

        state.recalculate_compute();

        // Unlock next tier
        unlock_next_hardware(state, kind);

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Upgrade,
            message: format!("Purchased {} for {}", kind.name(), formulas::format_cash(cost)),
        });
    }
}

fn unlock_next_hardware(state: &mut GameState, purchased: HardwareKind) {
    let next = match purchased {
        HardwareKind::UsedLaptop => Some("hw_refurb_desktop"),
        HardwareKind::RefurbishedDesktop => Some("hw_gaming_pc"),
        HardwareKind::GamingPC => Some("hw_workstation"),
        HardwareKind::Workstation => Some("hw_dual_gpu"),
        HardwareKind::DualGpuRig => Some("hw_server_rack"),
        HardwareKind::ServerRack => Some("hw_gpu_cluster"),
        HardwareKind::GpuCluster => Some("hw_data_center"),
        HardwareKind::DataCenter => None,
    };
    if let Some(id) = next {
        if !state.unlocked_upgrades.contains(&id.to_string()) {
            state.unlocked_upgrades.push(id.to_string());
        }
    }
    // Also unlock basic LLM after buying first hardware
    if !state.unlocked_upgrades.contains(&"llm_basic".to_string()) {
        state.unlocked_upgrades.push("llm_basic".to_string());
    }
}

fn buy_llm(state: &mut GameState, item_idx: usize) {
    let all = LlmTier::all();
    if item_idx >= all.len() {
        return;
    }
    let tier = all[item_idx];

    if !state.unlocked_upgrades.contains(&tier.unlock_id().to_string()) {
        return;
    }

    if tier == state.active_llm {
        return; // Already active
    }

    let cost = tier.unlock_cost();
    if state.cash >= cost {
        state.cash -= cost;
        state.active_llm = tier;

        // Unlock next tier
        let next = match tier {
            LlmTier::FreeTier => Some("llm_basic"),
            LlmTier::BasicSub => Some("llm_pro"),
            LlmTier::ProSub => Some("llm_team"),
            LlmTier::TeamSub => Some("llm_enterprise"),
            LlmTier::EnterpriseSub => Some("llm_custom"),
            LlmTier::CustomCluster => None,
        };
        if let Some(id) = next {
            if !state.unlocked_upgrades.contains(&id.to_string()) {
                state.unlocked_upgrades.push(id.to_string());
            }
        }

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Upgrade,
            message: format!("Upgraded to {} LLM", tier.name()),
        });
    }
}

fn hire_agent(state: &mut GameState) {
    if state.agents.len() as u32 >= state.max_agents + state.prestige_bonuses.extra_agent_slots {
        return;
    }

    let cost = formulas::agent_hire_cost(state.agents.len() as u32);
    if state.cash >= cost {
        state.cash -= cost;
        let id = state.agents.len() as u32;
        state.agents.push(Agent {
            id,
            name: format!("Agent-{}", id + 1),
            specialization: AgentSpec::Generalist,
            skill_level: 1.0,
            status: AgentStatus::Idle,
            current_project: None,
            lines_written: 0,
            bugs_introduced: 0,
        });

        // Unlock more agent slots at milestones
        let count = state.agents.len();
        if count >= 2 && state.max_agents < 3 {
            state.max_agents = 3;
        }
        if count >= 3 && state.max_agents < 5 {
            state.max_agents = 5;
        }
        if count >= 5 && state.max_agents < 10 {
            state.max_agents = 10;
        }
        if count >= 10 && state.max_agents < 25 {
            state.max_agents = 25;
        }
        if count >= 25 && state.max_agents < 100 {
            state.max_agents = 100;
        }

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::AgentHired,
            message: format!("Hired Agent-{} for {}", id + 1, formulas::format_cash(cost)),
        });
    }
}
