// Tech tree unlock logic
// Hardware and LLM unlocks handled in economy.rs
// Project type unlocks triggered by milestones
// Endgame LLM tiers unlocked by completing specific projects (handled in tick.rs)

use super::state::GameState;

pub fn check_milestone_unlocks(state: &mut GameState) {
    // Perk unlocks based on hardware ownership
    use super::state::HardwareKind;
    if state
        .hardware
        .iter()
        .any(|h| h.kind == HardwareKind::Workstation && h.count > 0)
    {
        add_unlock(state, "perk_ambient_audio");
    }
    if state
        .hardware
        .iter()
        .any(|h| h.kind == HardwareKind::ServerRack && h.count > 0)
    {
        add_unlock(state, "perk_radio");
    }

    let completed = state.completed_project_count;

    // Phase 0: Consultancy (existing)
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

    // Phase 1: Industry Expansion
    if completed >= 35 {
        add_unlock(state, "proj_healthcare");
    }
    if completed >= 40 {
        add_unlock(state, "proj_legal");
    }
    if completed >= 45 {
        add_unlock(state, "proj_financial");
    }
    if completed >= 50 {
        add_unlock(state, "proj_gov_contract");
    }
    if completed >= 55 {
        add_unlock(state, "proj_scientific");
    }
    if completed >= 60 {
        add_unlock(state, "proj_autonomous");
    }

    // Phase 2: Post-Human
    if completed >= 65 {
        add_unlock(state, "proj_agi_research");
        add_unlock(state, "spec_researcher");
    }
    if completed >= 70 {
        add_unlock(state, "proj_quantum_compiler");
    }
    if completed >= 75 {
        add_unlock(state, "proj_nanotech");
        add_unlock(state, "spec_bioengineer");
    }
    if completed >= 80 {
        add_unlock(state, "proj_humanoid");
    }
    if completed >= 85 {
        add_unlock(state, "proj_robotics_factory");
        add_unlock(state, "spec_humanoid_worker");
    }
    if completed >= 90 {
        add_unlock(state, "spec_humanoid_engineer");
    }
    if completed >= 95 {
        add_unlock(state, "proj_consciousness");
    }

    // Phase 3: Space Program
    if completed >= 100 {
        add_unlock(state, "proj_launch_vehicle");
    }
    if completed >= 110 {
        add_unlock(state, "proj_orbital_station");
        add_unlock(state, "spec_orbital_drone");
    }
    if completed >= 115 {
        add_unlock(state, "proj_deep_space");
    }
    if completed >= 120 {
        add_unlock(state, "proj_asteroid_mining");
    }
    if completed >= 130 {
        add_unlock(state, "proj_mars_colony");
        add_unlock(state, "spec_deep_space_unit");
    }
    if completed >= 135 {
        add_unlock(state, "proj_interplanetary");
    }

    // Phase 4: Megastructures
    if completed >= 150 {
        add_unlock(state, "proj_dyson_segment");
    }
    if completed >= 155 {
        add_unlock(state, "spec_computronium_entity");
    }
    if completed >= 160 {
        add_unlock(state, "proj_computronium_slab");
    }
}

/// Milestone definitions for the tech tree modal display.
pub struct Milestone {
    pub projects_needed: u32,
    pub label: &'static str,
    pub phase_name: &'static str,
}

pub fn all_milestones() -> Vec<Milestone> {
    vec![
        // Phase 0: Consultancy
        Milestone {
            projects_needed: 0,
            label: "Landing Page, Personal Site, Script",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 3,
            label: "CRUD App, REST API",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 5,
            label: "Mobile App",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 8,
            label: "E-commerce",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 12,
            label: "SaaS Product",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 15,
            label: "Data Pipeline, Open Source",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 20,
            label: "ML Model, Enterprise SW",
            phase_name: "Consultancy",
        },
        Milestone {
            projects_needed: 30,
            label: "Crypto Protocol, Game Dev",
            phase_name: "Consultancy",
        },
        // Phase 1: Industry
        Milestone {
            projects_needed: 35,
            label: "Healthcare Portal",
            phase_name: "Industry",
        },
        Milestone {
            projects_needed: 40,
            label: "Legal Discovery AI",
            phase_name: "Industry",
        },
        Milestone {
            projects_needed: 45,
            label: "Financial Engine",
            phase_name: "Industry",
        },
        Milestone {
            projects_needed: 50,
            label: "Gov Contract",
            phase_name: "Industry",
        },
        Milestone {
            projects_needed: 55,
            label: "Scientific Sim",
            phase_name: "Industry",
        },
        Milestone {
            projects_needed: 60,
            label: "Autonomous Fleet",
            phase_name: "Industry",
        },
        // Phase 2: Post-Human
        Milestone {
            projects_needed: 65,
            label: "AGI Research + Researcher spec",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 70,
            label: "Quantum Compiler",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 75,
            label: "Nanotech Prototype + BioEngineer",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 80,
            label: "Humanoid Chassis",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 85,
            label: "Robotics Factory + Humanoid spec",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 90,
            label: "Humanoid Engineer spec",
            phase_name: "Post-Human",
        },
        Milestone {
            projects_needed: 95,
            label: "Consciousness Upload",
            phase_name: "Post-Human",
        },
        // Phase 3: Space
        Milestone {
            projects_needed: 100,
            label: "Launch Vehicle",
            phase_name: "Space Age",
        },
        Milestone {
            projects_needed: 110,
            label: "Orbital Station + Orbital Drone",
            phase_name: "Space Age",
        },
        Milestone {
            projects_needed: 115,
            label: "Deep Space Probe",
            phase_name: "Space Age",
        },
        Milestone {
            projects_needed: 120,
            label: "Asteroid Mining",
            phase_name: "Space Age",
        },
        Milestone {
            projects_needed: 130,
            label: "Mars Colony + Deep Space Unit",
            phase_name: "Space Age",
        },
        Milestone {
            projects_needed: 135,
            label: "Interplanetary Net",
            phase_name: "Space Age",
        },
        // Phase 4: Kardashev
        Milestone {
            projects_needed: 150,
            label: "Dyson Swarm Segment",
            phase_name: "Kardashev",
        },
        Milestone {
            projects_needed: 155,
            label: "Computronium Entity spec",
            phase_name: "Kardashev",
        },
        Milestone {
            projects_needed: 160,
            label: "Computronium Slab",
            phase_name: "Kardashev",
        },
    ]
}

fn add_unlock(state: &mut GameState, id: &str) {
    if !state.unlocked_upgrades.contains(&id.to_string()) {
        state.unlocked_upgrades.push(id.to_string());
    }
}
