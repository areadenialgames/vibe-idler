use rand::Rng;

use super::formulas;
use super::state::*;

pub fn generate_contract(state: &GameState, rng: &mut impl Rng) -> Project {
    // Pick from unlocked project types
    let available_kinds = get_available_kinds(state);
    let kind = available_kinds[rng.gen_range(0..available_kinds.len())];
    let difficulty = rng.gen_range(1..=3).min(5); // Early game: low difficulty

    let client_name = crate::data::project_names::random_client(rng);
    let project_name = crate::data::project_names::random_project(rng, kind);

    let work = formulas::project_work_required(kind.base_work(), difficulty);
    let payment = if kind.is_recurring() {
        let monthly = kind.base_payment() * difficulty as f64 * 0.15;
        ProjectPayment::Recurring { monthly }
    } else {
        ProjectPayment::OneTime(formulas::project_payment(kind.base_payment(), difficulty, 0.0))
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
    let mut kinds = vec![];
    let u = &state.unlocked_upgrades;

    if u.iter().any(|s| s == "proj_landing") { kinds.push(ProjectKind::LandingPage); }
    if u.iter().any(|s| s == "proj_personal_site") { kinds.push(ProjectKind::PersonalWebsite); }
    if u.iter().any(|s| s == "proj_simple_script") { kinds.push(ProjectKind::SimpleScript); }
    if u.iter().any(|s| s == "proj_crud_app") { kinds.push(ProjectKind::CrudApp); }
    if u.iter().any(|s| s == "proj_rest_api") { kinds.push(ProjectKind::RestApi); }
    if u.iter().any(|s| s == "proj_mobile_app") { kinds.push(ProjectKind::MobileApp); }
    if u.iter().any(|s| s == "proj_ecommerce") { kinds.push(ProjectKind::EcommerceSite); }
    if u.iter().any(|s| s == "proj_saas") { kinds.push(ProjectKind::SaasProduct); }
    if u.iter().any(|s| s == "proj_data_pipeline") { kinds.push(ProjectKind::DataPipeline); }
    if u.iter().any(|s| s == "proj_ml_model") { kinds.push(ProjectKind::MlModel); }
    if u.iter().any(|s| s == "proj_enterprise") { kinds.push(ProjectKind::EnterpriseSoftware); }
    if u.iter().any(|s| s == "proj_open_source") { kinds.push(ProjectKind::OpenSourceFramework); }
    if u.iter().any(|s| s == "proj_crypto") { kinds.push(ProjectKind::CryptoProtocol); }
    if u.iter().any(|s| s == "proj_gamedev") { kinds.push(ProjectKind::GameDev); }

    if kinds.is_empty() {
        kinds.push(ProjectKind::LandingPage);
    }
    kinds
}
