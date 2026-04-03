pub fn hardware_cost(base: f64, growth_rate: f64, owned: u32, prestige_reduction: f64) -> f64 {
    base * growth_rate.powi(owned as i32) * (1.0 - prestige_reduction)
}

pub fn agent_hire_cost(agents_owned: u32) -> f64 {
    100.0 * 1.5_f64.powi(agents_owned as i32)
}

pub fn agent_work_speed(
    llm_quality: f64,
    total_compute: f64,
    spec_matches: bool,
    prestige_speed_mult: f64,
    skill_level: f64,
    has_architect: bool,
) -> f64 {
    let base_speed = 0.1;
    let hw_factor = (1.0 + total_compute).log2() / 5.0;
    let spec_bonus = if spec_matches { 1.5 } else { 1.0 };
    let arch_bonus = if has_architect { 1.25 } else { 1.0 };
    base_speed * llm_quality * hw_factor * spec_bonus * prestige_speed_mult * skill_level * arch_bonus
}

pub fn project_work_required(base_work: f64, difficulty: u32) -> f64 {
    base_work * (difficulty as f64).powf(1.8)
}

pub fn project_payment(base_payment: f64, difficulty: u32, reputation_bonus: f64) -> f64 {
    base_payment * (difficulty as f64).powi(2) * (1.0 + reputation_bonus)
}

pub fn bug_chance(llm_quality: f64, difficulty: u32, bug_reduction: f64) -> f64 {
    (0.0004 * (1.0 / llm_quality) * (difficulty as f64 / 5.0) * (1.0 - bug_reduction)).max(0.0)
}

pub fn prestige_reputation_earned(lifetime_cash_this_run: f64) -> f64 {
    if lifetime_cash_this_run < 10_000.0 {
        return 0.0;
    }
    (lifetime_cash_this_run / 10_000.0).sqrt().floor()
}

pub fn format_cash(amount: f64) -> String {
    if amount >= 1_000_000.0 {
        format!("${:.2}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("${:.1}K", amount / 1_000.0)
    } else {
        format!("${:.2}", amount)
    }
}
