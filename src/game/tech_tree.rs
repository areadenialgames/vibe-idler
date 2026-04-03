// Tech tree unlock logic
// Hardware and LLM unlocks handled in economy.rs
// Project type unlocks triggered by milestones

use super::state::GameState;

pub fn check_milestone_unlocks(state: &mut GameState) {
    let completed = state.completed_project_count;

    // Project unlocks based on completed projects
    if completed >= 3 {
        add_unlock(state, "proj_crud_app");
        add_unlock(state, "proj_rest_api");
    }
    if completed >= 5 {
        add_unlock(state, "proj_mobile_app");
    }
    if completed >= 8 {
        add_unlock(state, "proj_ecommerce");
    }
    if completed >= 12 {
        add_unlock(state, "proj_saas");
    }
    if completed >= 15 {
        add_unlock(state, "proj_data_pipeline");
        add_unlock(state, "proj_open_source");
    }
    if completed >= 20 {
        add_unlock(state, "proj_ml_model");
        add_unlock(state, "proj_enterprise");
    }
    if completed >= 30 {
        add_unlock(state, "proj_crypto");
        add_unlock(state, "proj_gamedev");
    }
}

fn add_unlock(state: &mut GameState, id: &str) {
    if !state.unlocked_upgrades.contains(&id.to_string()) {
        state.unlocked_upgrades.push(id.to_string());
    }
}
