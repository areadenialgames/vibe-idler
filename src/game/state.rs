use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub cash: f64,
    pub lifetime_cash: f64,
    pub reputation: f64,
    pub pivot_count: u32,
    pub total_ticks: u64,

    pub hardware: Vec<OwnedHardware>,
    pub total_compute: f64,
    pub active_llm: LlmTier,

    pub agents: Vec<Agent>,
    pub max_agents: u32,

    pub active_projects: Vec<Project>,
    pub completed_project_count: u32,
    pub available_contracts: Vec<Project>,
    pub passive_income_sources: Vec<PassiveIncome>,

    pub unlocked_upgrades: Vec<String>,

    pub event_log: Vec<GameEvent>,
    pub commit_log: Vec<CommitEntry>,

    pub income_history: Vec<f64>,
    pub expense_history: Vec<f64>,

    #[serde(default)]
    pub income_accumulator: f64,
    #[serde(default)]
    pub expense_accumulator: f64,

    pub prestige_bonuses: PrestigeBonuses,

    #[serde(default = "default_true")]
    pub audio_enabled: bool,
    #[serde(default = "default_true")]
    pub radio_enabled: bool,
    #[serde(default)]
    pub radio_station: usize,
}

fn default_true() -> bool {
    true
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            cash: 500.0,
            lifetime_cash: 0.0,
            reputation: 0.0,
            pivot_count: 0,
            total_ticks: 0,
            hardware: vec![],
            total_compute: 0.0,
            active_llm: LlmTier::FreeTier,
            agents: vec![],
            max_agents: 2,
            active_projects: vec![],
            completed_project_count: 0,
            available_contracts: vec![],
            passive_income_sources: vec![],
            unlocked_upgrades: vec![
                "hw_used_laptop".into(),
                "llm_free".into(),
                "proj_landing".into(),
                "proj_personal_site".into(),
                "proj_simple_script".into(),
            ],
            event_log: vec![],
            commit_log: vec![],
            income_history: vec![0.0; 60],
            expense_history: vec![0.0; 60],
            income_accumulator: 0.0,
            expense_accumulator: 0.0,
            prestige_bonuses: PrestigeBonuses::default(),
            audio_enabled: true,
            radio_enabled: true,
            radio_station: 0,
        };

        // Start with one generalist agent
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

        state.event_log.push(GameEvent {
            tick: 0,
            kind: EventKind::RandomEvent,
            message: "Welcome to Vibe Idler! Open the [S]hop to buy hardware and get started.".into(),
        });

        state
    }

    pub fn recalculate_compute(&mut self) {
        self.total_compute = self.hardware.iter().map(|h| h.kind.compute() * h.count as f64).sum();
    }

    pub fn income_per_tick(&self) -> f64 {
        let monthly: f64 = self.passive_income_sources.iter().map(|p| p.monthly_income).sum();
        monthly / TICKS_PER_GAME_MONTH as f64
    }

    pub fn expense_per_month(&self) -> f64 {
        let llm = self.active_llm.monthly_cost();
        let agents = self.agents.len() as f64 * 10.0;
        let hw_maintenance: f64 = self.hardware.iter()
            .map(|h| h.kind.base_cost() * 0.02 * h.count as f64)
            .sum();
        llm + agents + hw_maintenance
    }
}

pub const TICKS_PER_GAME_MONTH: u64 = 3000;

#[derive(Serialize, Deserialize, Clone)]
pub struct OwnedHardware {
    pub kind: HardwareKind,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Copy)]
pub enum HardwareKind {
    UsedLaptop,
    RefurbishedDesktop,
    GamingPC,
    Workstation,
    DualGpuRig,
    ServerRack,
    GpuCluster,
    DataCenter,
}

impl HardwareKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::UsedLaptop => "Used Laptop",
            Self::RefurbishedDesktop => "Refurb Desktop",
            Self::GamingPC => "Gaming PC",
            Self::Workstation => "Workstation",
            Self::DualGpuRig => "Dual GPU Rig",
            Self::ServerRack => "Server Rack",
            Self::GpuCluster => "GPU Cluster",
            Self::DataCenter => "Data Center",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            Self::UsedLaptop => 200.0,
            Self::RefurbishedDesktop => 500.0,
            Self::GamingPC => 1500.0,
            Self::Workstation => 5000.0,
            Self::DualGpuRig => 15000.0,
            Self::ServerRack => 50000.0,
            Self::GpuCluster => 200000.0,
            Self::DataCenter => 1000000.0,
        }
    }

    pub fn growth_rate(&self) -> f64 {
        match self {
            Self::UsedLaptop => 1.15,
            Self::RefurbishedDesktop => 1.18,
            Self::GamingPC => 1.20,
            Self::Workstation => 1.22,
            Self::DualGpuRig => 1.25,
            Self::ServerRack => 1.28,
            Self::GpuCluster => 1.30,
            Self::DataCenter => 1.35,
        }
    }

    pub fn compute(&self) -> f64 {
        match self {
            Self::UsedLaptop => 1.0,
            Self::RefurbishedDesktop => 3.0,
            Self::GamingPC => 10.0,
            Self::Workstation => 30.0,
            Self::DualGpuRig => 80.0,
            Self::ServerRack => 250.0,
            Self::GpuCluster => 1000.0,
            Self::DataCenter => 5000.0,
        }
    }

    pub fn unlock_id(&self) -> &'static str {
        match self {
            Self::UsedLaptop => "hw_used_laptop",
            Self::RefurbishedDesktop => "hw_refurb_desktop",
            Self::GamingPC => "hw_gaming_pc",
            Self::Workstation => "hw_workstation",
            Self::DualGpuRig => "hw_dual_gpu",
            Self::ServerRack => "hw_server_rack",
            Self::GpuCluster => "hw_gpu_cluster",
            Self::DataCenter => "hw_data_center",
        }
    }

    pub fn all() -> &'static [HardwareKind] {
        &[
            Self::UsedLaptop,
            Self::RefurbishedDesktop,
            Self::GamingPC,
            Self::Workstation,
            Self::DualGpuRig,
            Self::ServerRack,
            Self::GpuCluster,
            Self::DataCenter,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum LlmTier {
    FreeTier,
    BasicSub,
    ProSub,
    TeamSub,
    EnterpriseSub,
    CustomCluster,
}

impl LlmTier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FreeTier => "Free Tier",
            Self::BasicSub => "Basic",
            Self::ProSub => "Pro",
            Self::TeamSub => "Team",
            Self::EnterpriseSub => "Enterprise",
            Self::CustomCluster => "Custom Cluster",
        }
    }

    pub fn quality(&self) -> f64 {
        match self {
            Self::FreeTier => 0.3,
            Self::BasicSub => 1.0,
            Self::ProSub => 1.8,
            Self::TeamSub => 2.5,
            Self::EnterpriseSub => 4.0,
            Self::CustomCluster => 6.0,
        }
    }

    pub fn monthly_cost(&self) -> f64 {
        match self {
            Self::FreeTier => 0.0,
            Self::BasicSub => 20.0,
            Self::ProSub => 50.0,
            Self::TeamSub => 200.0,
            Self::EnterpriseSub => 1000.0,
            Self::CustomCluster => 5000.0,
        }
    }

    pub fn unlock_cost(&self) -> f64 {
        match self {
            Self::FreeTier => 0.0,
            Self::BasicSub => 50.0,
            Self::ProSub => 200.0,
            Self::TeamSub => 1000.0,
            Self::EnterpriseSub => 5000.0,
            Self::CustomCluster => 50000.0,
        }
    }

    pub fn unlock_id(&self) -> &'static str {
        match self {
            Self::FreeTier => "llm_free",
            Self::BasicSub => "llm_basic",
            Self::ProSub => "llm_pro",
            Self::TeamSub => "llm_team",
            Self::EnterpriseSub => "llm_enterprise",
            Self::CustomCluster => "llm_custom",
        }
    }

    pub fn all() -> &'static [LlmTier] {
        &[
            Self::FreeTier,
            Self::BasicSub,
            Self::ProSub,
            Self::TeamSub,
            Self::EnterpriseSub,
            Self::CustomCluster,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub specialization: AgentSpec,
    pub skill_level: f64,
    pub status: AgentStatus,
    pub current_project: Option<usize>,
    pub lines_written: u64,
    pub bugs_introduced: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum AgentSpec {
    Generalist,
    Frontend,
    Backend,
    Mobile,
    DevOps,
    DataScience,
    Security,
    Architect,
}

impl AgentSpec {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Generalist => "Generalist",
            Self::Frontend => "Frontend",
            Self::Backend => "Backend",
            Self::Mobile => "Mobile",
            Self::DevOps => "DevOps",
            Self::DataScience => "Data Sci",
            Self::Security => "Security",
            Self::Architect => "Architect",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AgentStatus {
    Idle,
    Working,
    Debugging,
}

#[allow(dead_code)]
impl AgentStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Working => "Working",
            Self::Debugging => "Debugging",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub kind: ProjectKind,
    pub difficulty: u32,
    pub progress: f64,
    pub total_work_units: f64,
    pub work_done: f64,
    pub payment: ProjectPayment,
    pub bug_count: u32,
    pub assigned_agents: Vec<u32>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum ProjectKind {
    LandingPage,
    PersonalWebsite,
    SimpleScript,
    CrudApp,
    RestApi,
    MobileApp,
    EcommerceSite,
    SaasProduct,
    DataPipeline,
    MlModel,
    EnterpriseSoftware,
    OpenSourceFramework,
    CryptoProtocol,
    GameDev,
}

#[allow(dead_code)]
impl ProjectKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LandingPage => "Landing Page",
            Self::PersonalWebsite => "Personal Site",
            Self::SimpleScript => "Script",
            Self::CrudApp => "CRUD App",
            Self::RestApi => "REST API",
            Self::MobileApp => "Mobile App",
            Self::EcommerceSite => "E-commerce",
            Self::SaasProduct => "SaaS Product",
            Self::DataPipeline => "Data Pipeline",
            Self::MlModel => "ML Model",
            Self::EnterpriseSoftware => "Enterprise SW",
            Self::OpenSourceFramework => "Open Source",
            Self::CryptoProtocol => "Crypto Protocol",
            Self::GameDev => "Game Dev",
        }
    }

    pub fn base_work(&self) -> f64 {
        match self {
            Self::LandingPage => 50.0,
            Self::PersonalWebsite => 80.0,
            Self::SimpleScript => 30.0,
            Self::CrudApp => 200.0,
            Self::RestApi => 150.0,
            Self::MobileApp => 400.0,
            Self::EcommerceSite => 600.0,
            Self::SaasProduct => 1000.0,
            Self::DataPipeline => 500.0,
            Self::MlModel => 800.0,
            Self::EnterpriseSoftware => 3000.0,
            Self::OpenSourceFramework => 2000.0,
            Self::CryptoProtocol => 1500.0,
            Self::GameDev => 2500.0,
        }
    }

    pub fn base_payment(&self) -> f64 {
        match self {
            Self::LandingPage => 100.0,
            Self::PersonalWebsite => 150.0,
            Self::SimpleScript => 50.0,
            Self::CrudApp => 500.0,
            Self::RestApi => 400.0,
            Self::MobileApp => 1200.0,
            Self::EcommerceSite => 2000.0,
            Self::SaasProduct => 3000.0,
            Self::DataPipeline => 1500.0,
            Self::MlModel => 2500.0,
            Self::EnterpriseSoftware => 15000.0,
            Self::OpenSourceFramework => 500.0,
            Self::CryptoProtocol => 5000.0,
            Self::GameDev => 4000.0,
        }
    }

    pub fn is_recurring(&self) -> bool {
        matches!(self, Self::SaasProduct | Self::OpenSourceFramework)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProjectPayment {
    OneTime(f64),
    Recurring { monthly: f64 },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PassiveIncome {
    pub source_name: String,
    pub monthly_income: f64,
    pub months_active: u32,
    pub churn_rate: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameEvent {
    pub tick: u64,
    pub kind: EventKind,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EventKind {
    ProjectCompleted,
    BugFound,
    ClientMessage,
    Achievement,
    Upgrade,
    AgentHired,
    Income,
    Expense,
    RandomEvent,
}

impl EventKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::ProjectCompleted => ">>",
            Self::BugFound => "!!",
            Self::ClientMessage => "<<",
            Self::Achievement => "**",
            Self::Upgrade => "++",
            Self::AgentHired => "++",
            Self::Income => "$$",
            Self::Expense => "--",
            Self::RandomEvent => "::",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommitEntry {
    pub tick: u64,
    pub agent_name: String,
    pub project_name: String,
    pub message: String,
    pub additions: u32,
    pub deletions: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrestigeBonuses {
    pub income_multiplier: f64,
    pub agent_speed_multiplier: f64,
    pub cost_reduction: f64,
    pub starting_cash_bonus: f64,
    pub extra_agent_slots: u32,
}

impl Default for PrestigeBonuses {
    fn default() -> Self {
        Self {
            income_multiplier: 1.0,
            agent_speed_multiplier: 1.0,
            cost_reduction: 0.0,
            starting_cash_bonus: 0.0,
            extra_agent_slots: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum PerkKind {
    OfficeAmbiance,
    StreamingSub,
}

impl PerkKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::OfficeAmbiance => "Office Soundscape",
            Self::StreamingSub => "Premium Streaming Sub",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::OfficeAmbiance => "Ambient background audio for your workspace. Mechanical keyboards, humming servers, and the occasional coffee grinder.",
            Self::StreamingSub => "A music streaming subscription for the whole office. Keeps morale high during crunch time.",
        }
    }

    pub fn cost(&self) -> f64 {
        match self {
            Self::OfficeAmbiance => 8000.0,
            Self::StreamingSub => 75000.0,
        }
    }

    pub fn unlock_id(&self) -> &'static str {
        match self {
            Self::OfficeAmbiance => "perk_ambient_audio",
            Self::StreamingSub => "perk_radio",
        }
    }

    pub fn owned_id(&self) -> String {
        format!("{}_owned", self.unlock_id())
    }

    pub fn all() -> &'static [PerkKind] {
        &[Self::OfficeAmbiance, Self::StreamingSub]
    }
}
