use super::formulas;
use super::state::*;

pub fn try_purchase(state: &mut GameState, tab: usize, item_idx: usize) {
    match tab {
        0 => buy_hardware(state, item_idx),
        1 => buy_llm(state, item_idx),
        2 => hire_agent(state),
        3 => buy_perk(state, item_idx),
        4 => hire_robot(state, item_idx),      // Robotics tab
        5 => hire_space_unit(state, item_idx), // Space tab
        _ => {}
    }
}

fn buy_hardware(state: &mut GameState, item_idx: usize) {
    // Only show unlocked hardware
    let visible: Vec<HardwareKind> = HardwareKind::all()
        .iter()
        .filter(|k| state.unlocked_upgrades.contains(&k.unlock_id().to_string()))
        .copied()
        .collect();

    if item_idx >= visible.len() {
        return;
    }
    let kind = visible[item_idx];

    let owned = state
        .hardware
        .iter()
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
            message: format!(
                "Purchased {} for {}",
                kind.name(),
                formulas::format_cash(cost)
            ),
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
        HardwareKind::DataCenter => Some("hw_quantum_processor"),
        HardwareKind::QuantumProcessor => Some("hw_neural_fabric"),
        HardwareKind::NeuralFabric => Some("hw_nanotech_assembler"),
        HardwareKind::NanotechAssembler => Some("hw_orbital_compute"),
        HardwareKind::OrbitalCompute => Some("hw_asteroid_foundry"),
        HardwareKind::AsteroidFoundry => Some("hw_dyson_collector"),
        HardwareKind::DysonCollector => Some("hw_computronium_core"),
        HardwareKind::ComputroniumCore => None,
    };

    // Unlock perks based on hardware progression
    match purchased {
        HardwareKind::Workstation => add_unlock(state, "perk_ambient_audio"),
        HardwareKind::ServerRack => add_unlock(state, "perk_radio"),
        _ => {}
    }
    if let Some(id) = next {
        add_unlock(state, id);
    }
    // Also unlock basic LLM after buying first hardware
    add_unlock(state, "llm_basic");
}

fn buy_llm(state: &mut GameState, item_idx: usize) {
    // Only show unlocked LLM tiers
    let visible: Vec<LlmTier> = LlmTier::all()
        .iter()
        .filter(|t| state.unlocked_upgrades.contains(&t.unlock_id().to_string()))
        .copied()
        .collect();

    if item_idx >= visible.len() {
        return;
    }
    let tier = visible[item_idx];

    if tier == state.active_llm {
        return; // Already active
    }

    let cost = tier.unlock_cost();
    if state.cash >= cost {
        state.cash -= cost;
        state.active_llm = tier;

        // Unlock next tier (only for the base chain; endgame tiers unlocked via projects)
        let next = match tier {
            LlmTier::FreeTier => Some("llm_basic"),
            LlmTier::BasicSub => Some("llm_pro"),
            LlmTier::ProSub => Some("llm_team"),
            LlmTier::TeamSub => Some("llm_enterprise"),
            LlmTier::EnterpriseSub => Some("llm_custom"),
            // Endgame LLM tiers are unlocked by completing specific projects,
            // not by purchasing the previous tier.
            _ => None,
        };
        if let Some(id) = next {
            add_unlock(state, id);
        }

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Upgrade,
            message: format!("Upgraded to {} LLM", tier.name()),
        });
    }
}

fn hire_agent(state: &mut GameState) {
    let sw_count = state.agent_count_by_class(AgentClass::Software);
    let max = state.max_for_class(AgentClass::Software);
    if sw_count >= max {
        return;
    }

    let cost = formulas::agent_hire_cost(AgentClass::Software, sw_count);
    if state.cash >= cost {
        state.cash -= cost;
        let id = state.agents.len() as u32;
        state.agents.push(Agent {
            id,
            name: format!("Agent-{}", id + 1),
            specialization: AgentSpec::Generalist,
            agent_class: AgentClass::Software,
            skill_level: 1.0,
            status: AgentStatus::Idle,
            current_project: None,
            lines_written: 0,
            bugs_introduced: 0,
        });

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::AgentHired,
            message: format!(
                "Spun up Agent-{} for {}",
                id + 1,
                formulas::format_cash(cost)
            ),
        });
    }
}

fn hire_robot(state: &mut GameState, item_idx: usize) {
    // item_idx 0 = HumanoidWorker, 1 = HumanoidEngineer
    let (spec, class) = match item_idx {
        0 => (AgentSpec::HumanoidWorker, AgentClass::Humanoid),
        1 => (AgentSpec::HumanoidEngineer, AgentClass::Humanoid),
        _ => return,
    };

    let count = state.agent_count_by_class(class);
    let max = state.max_for_class(class);
    if count >= max {
        return;
    }

    let cost = formulas::agent_hire_cost(class, count);
    if state.cash >= cost {
        state.cash -= cost;
        let id = state.agents.len() as u32;
        let num = count + 1;
        state.agents.push(Agent {
            id,
            name: format!("{}-{}", class.name_prefix(), num),
            specialization: spec,
            agent_class: class,
            skill_level: 1.0,
            status: AgentStatus::Idle,
            current_project: None,
            lines_written: 0,
            bugs_introduced: 0,
        });

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::AgentHired,
            message: format!(
                "Built {}-{} for {}",
                class.name_prefix(),
                num,
                formulas::format_cash(cost)
            ),
        });
    }
}

fn hire_space_unit(state: &mut GameState, item_idx: usize) {
    // item_idx 0 = OrbitalDrone, 1 = DeepSpaceUnit, 2 = ComputroniumEntity
    let (spec, class) = match item_idx {
        0 => (AgentSpec::OrbitalDrone, AgentClass::SpaceDrone),
        1 => (AgentSpec::DeepSpaceUnit, AgentClass::SpaceDrone),
        2 => (AgentSpec::ComputroniumEntity, AgentClass::Computronium),
        _ => return,
    };

    let count = state.agent_count_by_class(class);
    let max = state.max_for_class(class);
    if count >= max {
        return;
    }

    let cost = formulas::agent_hire_cost(class, count);
    if state.cash >= cost {
        state.cash -= cost;
        let id = state.agents.len() as u32;
        let num = count + 1;
        state.agents.push(Agent {
            id,
            name: format!("{}-{}", class.name_prefix(), num),
            specialization: spec,
            agent_class: class,
            skill_level: 1.0,
            status: AgentStatus::Idle,
            current_project: None,
            lines_written: 0,
            bugs_introduced: 0,
        });

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::AgentHired,
            message: format!(
                "Deployed {}-{} for {}",
                class.name_prefix(),
                num,
                formulas::format_cash(cost)
            ),
        });
    }
}

fn buy_perk(state: &mut GameState, item_idx: usize) {
    let all = PerkKind::all();
    if item_idx >= all.len() {
        return;
    }
    let perk = all[item_idx];

    // Must be visible (unlocked) and not already owned
    if !state
        .unlocked_upgrades
        .contains(&perk.unlock_id().to_string())
    {
        return;
    }
    if state.unlocked_upgrades.contains(&perk.owned_id()) {
        return;
    }

    let cost = perk.cost();
    if state.cash >= cost {
        state.cash -= cost;
        state.unlocked_upgrades.push(perk.owned_id());

        state.event_log.push(GameEvent {
            tick: state.total_ticks,
            kind: EventKind::Upgrade,
            message: format!(
                "Installed {} for {}",
                perk.name(),
                formulas::format_cash(cost)
            ),
        });
    }
}

fn add_unlock(state: &mut GameState, id: &str) {
    if !state.unlocked_upgrades.contains(&id.to_string()) {
        state.unlocked_upgrades.push(id.to_string());
    }
}
