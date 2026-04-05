use rand::Rng;

use super::formulas;
use super::state::*;

pub fn generate_contract(state: &GameState, rng: &mut impl Rng) -> Project {
    // Pick from unlocked project types
    let available_kinds = get_available_kinds(state);
    let kind = available_kinds[rng.gen_range(0..available_kinds.len())];
    let difficulty = rng.gen_range(1..=3).min(5);

    let client_name = crate::data::project_names::random_client(rng, state.phase);
    let project_name = crate::data::project_names::random_project(rng, kind);

    let rep_bonus = state.prestige_bonuses.income_multiplier - 1.0;
    let work = formulas::project_work_required(kind.base_work(), difficulty);
    let payment = if kind.is_recurring() {
        let monthly = kind.base_payment() * difficulty as f64 * 0.15 * (1.0 + rep_bonus);
        ProjectPayment::Recurring { monthly }
    } else {
        ProjectPayment::OneTime(formulas::project_payment(
            kind.base_payment(),
            difficulty,
            rep_bonus,
        ))
    };

    Project {
        name: format!("{} ({})", project_name, client_name),
        kind,
        difficulty,
        progress: 0.0,
        total_work_units: work,
        work_done: 0.0,
        payment,
        bug_count: 0,
        assigned_agents: vec![],
    }
}

fn get_available_kinds(state: &GameState) -> Vec<ProjectKind> {
    // All project kinds paired with their unlock_id — just check unlock status
    let all_pairs: &[(ProjectKind, &str)] = &[
        // Phase 0
        (ProjectKind::LandingPage, "proj_landing"),
        (ProjectKind::PersonalWebsite, "proj_personal_site"),
        (ProjectKind::SimpleScript, "proj_simple_script"),
        (ProjectKind::CrudApp, "proj_crud_app"),
        (ProjectKind::RestApi, "proj_rest_api"),
        (ProjectKind::MobileApp, "proj_mobile_app"),
        (ProjectKind::EcommerceSite, "proj_ecommerce"),
        (ProjectKind::SaasProduct, "proj_saas"),
        (ProjectKind::DataPipeline, "proj_data_pipeline"),
        (ProjectKind::MlModel, "proj_ml_model"),
        (ProjectKind::EnterpriseSoftware, "proj_enterprise"),
        (ProjectKind::OpenSourceFramework, "proj_open_source"),
        (ProjectKind::CryptoProtocol, "proj_crypto"),
        (ProjectKind::GameDev, "proj_gamedev"),
        // Phase 1
        (ProjectKind::HealthcarePortal, "proj_healthcare"),
        (ProjectKind::LegalDiscovery, "proj_legal"),
        (ProjectKind::FinancialEngine, "proj_financial"),
        (ProjectKind::GovContract, "proj_gov_contract"),
        (ProjectKind::ScientificSim, "proj_scientific"),
        (ProjectKind::AutonomousFleet, "proj_autonomous"),
        // Phase 2
        (ProjectKind::AgiResearch, "proj_agi_research"),
        (ProjectKind::QuantumCompiler, "proj_quantum_compiler"),
        (ProjectKind::NanotechPrototype, "proj_nanotech"),
        (ProjectKind::HumanoidChassis, "proj_humanoid"),
        (ProjectKind::RoboticsFactory, "proj_robotics_factory"),
        (ProjectKind::ConsciousnessUpload, "proj_consciousness"),
        // Phase 3
        (ProjectKind::LaunchVehicle, "proj_launch_vehicle"),
        (ProjectKind::OrbitalStation, "proj_orbital_station"),
        (ProjectKind::DeepSpaceProbe, "proj_deep_space"),
        (ProjectKind::AsteroidMining, "proj_asteroid_mining"),
        (ProjectKind::MarsColony, "proj_mars_colony"),
        (ProjectKind::InterplanetaryNet, "proj_interplanetary"),
        // Phase 4
        (ProjectKind::DysonSwarmSegment, "proj_dyson_segment"),
        (ProjectKind::ComputroniumSlab, "proj_computronium_slab"),
    ];

    let kinds: Vec<ProjectKind> = all_pairs
        .iter()
        .filter(|(_, id)| state.unlocked_upgrades.iter().any(|s| s == id))
        .filter(|(k, _)| match k {
            ProjectKind::DysonSwarmSegment => state.mega_projects.dyson_segments_completed < 10,
            ProjectKind::ComputroniumSlab => state.mega_projects.planets_converted < 8,
            _ => true,
        })
        .map(|(k, _)| *k)
        .collect();

    if kinds.is_empty() {
        vec![ProjectKind::LandingPage]
    } else {
        kinds
    }
}
