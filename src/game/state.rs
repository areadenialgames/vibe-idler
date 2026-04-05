use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Game Phase
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GamePhase {
    #[default]
    Consultancy,
    Industry,
    PostHuman,
    SpaceAge,
    Kardashev,
    Victory,
}

impl GamePhase {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Consultancy => "VIBE IDLER",
            Self::Industry => "VIBE INDUSTRIES",
            Self::PostHuman => "NEXUS CORP",
            Self::SpaceAge => "NEXUS ORBITAL",
            Self::Kardashev => "DYSON PROJECT",
            Self::Victory => "TRANSCENDENCE",
        }
    }
}

// ---------------------------------------------------------------------------
// Mega-Projects (Dyson Sphere + Computronium)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MegaProjects {
    pub dyson_segments_completed: u32,
    pub dyson_sphere_progress: f64,
    pub dyson_sphere_complete: bool,
    pub planets_converted: u32,
    pub solar_conversion_progress: f64,
    pub solar_conversion_complete: bool,
    pub victory_achieved: bool,
}

// ---------------------------------------------------------------------------
// Core Game State
// ---------------------------------------------------------------------------

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

    // --- Endgame fields (all with serde defaults for save compat) ---
    #[serde(default)]
    pub phase: GamePhase,
    #[serde(default)]
    pub mega_projects: MegaProjects,
    #[serde(default)]
    pub max_humanoids: u32,
    #[serde(default)]
    pub max_space_drones: u32,
    #[serde(default)]
    pub max_computronium_units: u32,
    #[serde(default)]
    pub ascension_count: u32,
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
            phase: GamePhase::Consultancy,
            mega_projects: MegaProjects::default(),
            max_humanoids: 0,
            max_space_drones: 0,
            max_computronium_units: 0,
            ascension_count: 0,
        };

        // Start with one generalist agent
        state.agents.push(Agent {
            id: 0,
            name: "Agent-1".into(),
            specialization: AgentSpec::Generalist,
            agent_class: AgentClass::Software,
            skill_level: 1.0,
            status: AgentStatus::Idle,
            current_project: None,
            lines_written: 0,
            bugs_introduced: 0,
        });

        state.event_log.push(GameEvent {
            tick: 0,
            kind: EventKind::RandomEvent,
            message: "Welcome to Vibe Idler! Open the [S]hop to buy hardware and get started."
                .into(),
        });

        state
    }

    pub fn recalculate_compute(&mut self) {
        self.total_compute = self
            .hardware
            .iter()
            .map(|h| h.kind.compute() * h.count as f64)
            .sum();
    }

    pub fn recalculate_phase(&mut self) {
        let c = self.completed_project_count;
        self.phase = if self.mega_projects.victory_achieved {
            GamePhase::Victory
        } else if c >= 150 {
            GamePhase::Kardashev
        } else if c >= 100 {
            GamePhase::SpaceAge
        } else if c >= 65 {
            GamePhase::PostHuman
        } else if c >= 35 {
            GamePhase::Industry
        } else {
            GamePhase::Consultancy
        };
    }

    pub fn income_per_tick(&self) -> f64 {
        let monthly: f64 = self
            .passive_income_sources
            .iter()
            .map(|p| p.monthly_income)
            .sum();
        monthly / TICKS_PER_GAME_MONTH as f64
    }

    pub fn expense_per_month(&self) -> f64 {
        let llm = self.active_llm.monthly_cost();
        let agent_cost = self
            .agents
            .iter()
            .map(|a| a.agent_class.monthly_upkeep())
            .sum::<f64>();
        let hw_maintenance: f64 = self
            .hardware
            .iter()
            .map(|h| h.kind.base_cost() * 0.02 * h.count as f64)
            .sum();
        llm + agent_cost + hw_maintenance
    }

    pub fn visible_tab_count(&self) -> usize {
        let mut count = 4; // Hardware, LLM, Agents, Perks
        if self.phase >= GamePhase::PostHuman {
            count += 1; // Robotics
        }
        if self.phase >= GamePhase::SpaceAge {
            count += 1; // Space
        }
        count
    }

    pub fn agent_count_by_class(&self, class: AgentClass) -> u32 {
        self.agents
            .iter()
            .filter(|a| a.agent_class == class)
            .count() as u32
    }

    pub fn max_for_class(&self, class: AgentClass) -> u32 {
        match class {
            AgentClass::Software => self.max_agents + self.prestige_bonuses.extra_agent_slots,
            AgentClass::Humanoid => self.max_humanoids,
            AgentClass::SpaceDrone => self.max_space_drones,
            AgentClass::Computronium => self.max_computronium_units,
        }
    }
}

pub const TICKS_PER_GAME_MONTH: u64 = 3000;

// ---------------------------------------------------------------------------
// Hardware
// ---------------------------------------------------------------------------

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
    // Endgame tiers
    QuantumProcessor,
    NeuralFabric,
    NanotechAssembler,
    OrbitalCompute,
    AsteroidFoundry,
    DysonCollector,
    ComputroniumCore,
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
            Self::QuantumProcessor => "Quantum Processor",
            Self::NeuralFabric => "Neural Fabric",
            Self::NanotechAssembler => "Nanotech Assembler",
            Self::OrbitalCompute => "Orbital Compute",
            Self::AsteroidFoundry => "Asteroid Foundry",
            Self::DysonCollector => "Dyson Collector",
            Self::ComputroniumCore => "Computronium Core",
        }
    }

    pub fn base_cost(&self) -> f64 {
        match self {
            Self::UsedLaptop => 200.0,
            Self::RefurbishedDesktop => 500.0,
            Self::GamingPC => 1_500.0,
            Self::Workstation => 5_000.0,
            Self::DualGpuRig => 15_000.0,
            Self::ServerRack => 50_000.0,
            Self::GpuCluster => 200_000.0,
            Self::DataCenter => 1_000_000.0,
            Self::QuantumProcessor => 10_000_000.0,
            Self::NeuralFabric => 100_000_000.0,
            Self::NanotechAssembler => 1_000_000_000.0,
            Self::OrbitalCompute => 50_000_000_000.0,
            Self::AsteroidFoundry => 1_000_000_000_000.0,
            Self::DysonCollector => 100_000_000_000_000.0,
            Self::ComputroniumCore => 10_000_000_000_000_000.0,
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
            Self::QuantumProcessor => 1.38,
            Self::NeuralFabric => 1.40,
            Self::NanotechAssembler => 1.42,
            Self::OrbitalCompute => 1.45,
            Self::AsteroidFoundry => 1.48,
            Self::DysonCollector => 1.50,
            Self::ComputroniumCore => 1.55,
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
            Self::GpuCluster => 1_000.0,
            Self::DataCenter => 5_000.0,
            Self::QuantumProcessor => 25_000.0,
            Self::NeuralFabric => 150_000.0,
            Self::NanotechAssembler => 1_000_000.0,
            Self::OrbitalCompute => 10_000_000.0,
            Self::AsteroidFoundry => 100_000_000.0,
            Self::DysonCollector => 1_000_000_000.0,
            Self::ComputroniumCore => 50_000_000_000.0,
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
            Self::QuantumProcessor => "hw_quantum_processor",
            Self::NeuralFabric => "hw_neural_fabric",
            Self::NanotechAssembler => "hw_nanotech_assembler",
            Self::OrbitalCompute => "hw_orbital_compute",
            Self::AsteroidFoundry => "hw_asteroid_foundry",
            Self::DysonCollector => "hw_dyson_collector",
            Self::ComputroniumCore => "hw_computronium_core",
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
            Self::QuantumProcessor,
            Self::NeuralFabric,
            Self::NanotechAssembler,
            Self::OrbitalCompute,
            Self::AsteroidFoundry,
            Self::DysonCollector,
            Self::ComputroniumCore,
        ]
    }
}

// ---------------------------------------------------------------------------
// LLM Tiers
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum LlmTier {
    FreeTier,
    BasicSub,
    ProSub,
    TeamSub,
    EnterpriseSub,
    CustomCluster,
    // Endgame tiers
    AgiPrototype,
    Superintelligence,
    HiveMind,
    PlanetaryOvermind,
    MatrioshkaBrain,
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
            Self::AgiPrototype => "AGI Prototype",
            Self::Superintelligence => "Superintelligence",
            Self::HiveMind => "Hive Mind",
            Self::PlanetaryOvermind => "Planetary Overmind",
            Self::MatrioshkaBrain => "Matrioshka Brain",
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
            Self::AgiPrototype => 12.0,
            Self::Superintelligence => 25.0,
            Self::HiveMind => 60.0,
            Self::PlanetaryOvermind => 200.0,
            Self::MatrioshkaBrain => 1000.0,
        }
    }

    pub fn monthly_cost(&self) -> f64 {
        match self {
            Self::FreeTier => 0.0,
            Self::BasicSub => 20.0,
            Self::ProSub => 50.0,
            Self::TeamSub => 200.0,
            Self::EnterpriseSub => 1_000.0,
            Self::CustomCluster => 5_000.0,
            Self::AgiPrototype => 50_000.0,
            Self::Superintelligence => 500_000.0,
            Self::HiveMind => 10_000_000.0,
            Self::PlanetaryOvermind => 1_000_000_000.0,
            Self::MatrioshkaBrain => 0.0, // powered by the Dyson Sphere
        }
    }

    pub fn unlock_cost(&self) -> f64 {
        match self {
            Self::FreeTier => 0.0,
            Self::BasicSub => 50.0,
            Self::ProSub => 200.0,
            Self::TeamSub => 1_000.0,
            Self::EnterpriseSub => 5_000.0,
            Self::CustomCluster => 50_000.0,
            Self::AgiPrototype => 500_000.0,
            Self::Superintelligence => 5_000_000.0,
            Self::HiveMind => 100_000_000.0,
            Self::PlanetaryOvermind => 10_000_000_000.0,
            Self::MatrioshkaBrain => 1_000_000_000_000.0,
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
            Self::AgiPrototype => "llm_agi",
            Self::Superintelligence => "llm_asi",
            Self::HiveMind => "llm_hive_mind",
            Self::PlanetaryOvermind => "llm_overmind",
            Self::MatrioshkaBrain => "llm_matrioshka",
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
            Self::AgiPrototype,
            Self::Superintelligence,
            Self::HiveMind,
            Self::PlanetaryOvermind,
            Self::MatrioshkaBrain,
        ]
    }
}

// ---------------------------------------------------------------------------
// Agent Class (Software, Humanoid, SpaceDrone, Computronium)
// ---------------------------------------------------------------------------

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentClass {
    #[default]
    Software,
    Humanoid,
    SpaceDrone,
    Computronium,
}

#[allow(dead_code)]
impl AgentClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Software => "Software",
            Self::Humanoid => "Humanoid",
            Self::SpaceDrone => "Space Drone",
            Self::Computronium => "Computronium",
        }
    }

    pub fn hire_cost(&self, owned: u32) -> f64 {
        match self {
            Self::Software => 100.0 * 1.5_f64.powi(owned as i32),
            Self::Humanoid => 1_000_000.0 * 1.4_f64.powi(owned as i32),
            Self::SpaceDrone => 100_000_000.0 * 1.3_f64.powi(owned as i32),
            Self::Computronium => 10_000_000_000.0 * 1.2_f64.powi(owned as i32),
        }
    }

    pub fn monthly_upkeep(&self) -> f64 {
        match self {
            Self::Software => 10.0,
            Self::Humanoid => 10_000.0,
            Self::SpaceDrone => 1_000_000.0,
            Self::Computronium => 100_000_000.0,
        }
    }

    pub fn name_prefix(&self) -> &'static str {
        match self {
            Self::Software => "Agent",
            Self::Humanoid => "Robot",
            Self::SpaceDrone => "Drone",
            Self::Computronium => "Entity",
        }
    }
}

// ---------------------------------------------------------------------------
// Agents
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: u32,
    pub name: String,
    pub specialization: AgentSpec,
    #[serde(default)]
    pub agent_class: AgentClass,
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
    // Endgame specializations
    Researcher,
    BioEngineer,
    HumanoidWorker,
    HumanoidEngineer,
    OrbitalDrone,
    DeepSpaceUnit,
    ComputroniumEntity,
}

#[allow(dead_code)]
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
            Self::Researcher => "Researcher",
            Self::BioEngineer => "BioEngineer",
            Self::HumanoidWorker => "Humanoid",
            Self::HumanoidEngineer => "Humanoid Eng",
            Self::OrbitalDrone => "Orbital Drone",
            Self::DeepSpaceUnit => "Deep Space",
            Self::ComputroniumEntity => "Computronium",
        }
    }

    pub fn default_class(&self) -> AgentClass {
        match self {
            Self::HumanoidWorker | Self::HumanoidEngineer => AgentClass::Humanoid,
            Self::OrbitalDrone | Self::DeepSpaceUnit => AgentClass::SpaceDrone,
            Self::ComputroniumEntity => AgentClass::Computronium,
            _ => AgentClass::Software,
        }
    }

    /// Returns the work speed bonus this spec gets on the given project kind.
    pub fn spec_bonus(&self, kind: &ProjectKind) -> f64 {
        match self {
            Self::Architect => 1.25,
            Self::Researcher => match kind {
                ProjectKind::AgiResearch
                | ProjectKind::QuantumCompiler
                | ProjectKind::ScientificSim
                | ProjectKind::ConsciousnessUpload => 2.0,
                _ => 1.0,
            },
            Self::BioEngineer => match kind {
                ProjectKind::HealthcarePortal | ProjectKind::NanotechPrototype => 2.0,
                _ => 1.0,
            },
            Self::HumanoidWorker => match kind {
                ProjectKind::HumanoidChassis | ProjectKind::RoboticsFactory => 1.5,
                _ => 1.0,
            },
            Self::HumanoidEngineer => match kind {
                ProjectKind::LaunchVehicle
                | ProjectKind::OrbitalStation
                | ProjectKind::HumanoidChassis
                | ProjectKind::RoboticsFactory => 2.5,
                _ => 1.0,
            },
            Self::OrbitalDrone => match kind {
                ProjectKind::OrbitalStation
                | ProjectKind::DeepSpaceProbe
                | ProjectKind::AsteroidMining
                | ProjectKind::DysonSwarmSegment => 3.0,
                _ => 1.0,
            },
            Self::DeepSpaceUnit => match kind {
                ProjectKind::MarsColony
                | ProjectKind::InterplanetaryNet
                | ProjectKind::DysonSwarmSegment
                | ProjectKind::ComputroniumSlab => 4.0,
                _ => 1.0,
            },
            Self::ComputroniumEntity => 10.0, // bonus on everything
            _ => 1.0,
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

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------

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
    // Phase 0: Consultancy
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
    // Phase 1: Industry Expansion
    HealthcarePortal,
    LegalDiscovery,
    FinancialEngine,
    GovContract,
    ScientificSim,
    AutonomousFleet,
    // Phase 2: Post-Human
    AgiResearch,
    QuantumCompiler,
    NanotechPrototype,
    HumanoidChassis,
    RoboticsFactory,
    ConsciousnessUpload,
    // Phase 3: Space Program
    LaunchVehicle,
    OrbitalStation,
    DeepSpaceProbe,
    AsteroidMining,
    MarsColony,
    InterplanetaryNet,
    // Phase 4: Megastructures
    DysonSwarmSegment,
    ComputroniumSlab,
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
            // Phase 1
            Self::HealthcarePortal => "Healthcare Portal",
            Self::LegalDiscovery => "Legal Discovery AI",
            Self::FinancialEngine => "Financial Engine",
            Self::GovContract => "Gov Contract",
            Self::ScientificSim => "Scientific Sim",
            Self::AutonomousFleet => "Autonomous Fleet",
            // Phase 2
            Self::AgiResearch => "AGI Research",
            Self::QuantumCompiler => "Quantum Compiler",
            Self::NanotechPrototype => "Nanotech Prototype",
            Self::HumanoidChassis => "Humanoid Chassis",
            Self::RoboticsFactory => "Robotics Factory",
            Self::ConsciousnessUpload => "Consciousness Upload",
            // Phase 3
            Self::LaunchVehicle => "Launch Vehicle",
            Self::OrbitalStation => "Orbital Station",
            Self::DeepSpaceProbe => "Deep Space Probe",
            Self::AsteroidMining => "Asteroid Mining",
            Self::MarsColony => "Mars Colony",
            Self::InterplanetaryNet => "Interplanetary Net",
            // Phase 4
            Self::DysonSwarmSegment => "Dyson Segment",
            Self::ComputroniumSlab => "Computronium Slab",
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
            Self::SaasProduct => 1_000.0,
            Self::DataPipeline => 500.0,
            Self::MlModel => 800.0,
            Self::EnterpriseSoftware => 3_000.0,
            Self::OpenSourceFramework => 2_000.0,
            Self::CryptoProtocol => 1_500.0,
            Self::GameDev => 2_500.0,
            // Phase 1
            Self::HealthcarePortal => 5_000.0,
            Self::LegalDiscovery => 6_000.0,
            Self::FinancialEngine => 8_000.0,
            Self::GovContract => 12_000.0,
            Self::ScientificSim => 10_000.0,
            Self::AutonomousFleet => 15_000.0,
            // Phase 2
            Self::AgiResearch => 50_000.0,
            Self::QuantumCompiler => 60_000.0,
            Self::NanotechPrototype => 80_000.0,
            Self::HumanoidChassis => 100_000.0,
            Self::RoboticsFactory => 150_000.0,
            Self::ConsciousnessUpload => 200_000.0,
            // Phase 3
            Self::LaunchVehicle => 300_000.0,
            Self::OrbitalStation => 500_000.0,
            Self::DeepSpaceProbe => 600_000.0,
            Self::AsteroidMining => 800_000.0,
            Self::MarsColony => 1_500_000.0,
            Self::InterplanetaryNet => 1_000_000.0,
            // Phase 4
            Self::DysonSwarmSegment => 5_000_000.0,
            Self::ComputroniumSlab => 8_000_000.0,
        }
    }

    pub fn base_payment(&self) -> f64 {
        match self {
            Self::LandingPage => 100.0,
            Self::PersonalWebsite => 150.0,
            Self::SimpleScript => 50.0,
            Self::CrudApp => 500.0,
            Self::RestApi => 400.0,
            Self::MobileApp => 1_200.0,
            Self::EcommerceSite => 2_000.0,
            Self::SaasProduct => 3_000.0,
            Self::DataPipeline => 1_500.0,
            Self::MlModel => 2_500.0,
            Self::EnterpriseSoftware => 15_000.0,
            Self::OpenSourceFramework => 500.0,
            Self::CryptoProtocol => 5_000.0,
            Self::GameDev => 4_000.0,
            // Phase 1
            Self::HealthcarePortal => 25_000.0,
            Self::LegalDiscovery => 35_000.0,
            Self::FinancialEngine => 50_000.0,
            Self::GovContract => 100_000.0,
            Self::ScientificSim => 75_000.0,
            Self::AutonomousFleet => 200_000.0,
            // Phase 2
            Self::AgiResearch => 1_000_000.0,
            Self::QuantumCompiler => 2_000_000.0,
            Self::NanotechPrototype => 5_000_000.0,
            Self::HumanoidChassis => 10_000_000.0,
            Self::RoboticsFactory => 25_000_000.0,
            Self::ConsciousnessUpload => 50_000_000.0,
            // Phase 3
            Self::LaunchVehicle => 100_000_000.0,
            Self::OrbitalStation => 500_000_000.0,
            Self::DeepSpaceProbe => 1_000_000_000.0,
            Self::AsteroidMining => 2_000_000_000.0,
            Self::MarsColony => 10_000_000_000.0,
            Self::InterplanetaryNet => 5_000_000_000.0,
            // Phase 4
            Self::DysonSwarmSegment => 100_000_000_000.0,
            Self::ComputroniumSlab => 500_000_000_000.0,
        }
    }

    pub fn is_recurring(&self) -> bool {
        matches!(
            self,
            Self::SaasProduct
                | Self::OpenSourceFramework
                | Self::RoboticsFactory
                | Self::OrbitalStation
                | Self::AsteroidMining
                | Self::MarsColony
                | Self::InterplanetaryNet
        )
    }

    pub fn unlock_id(&self) -> &'static str {
        match self {
            Self::LandingPage => "proj_landing",
            Self::PersonalWebsite => "proj_personal_site",
            Self::SimpleScript => "proj_simple_script",
            Self::CrudApp => "proj_crud_app",
            Self::RestApi => "proj_rest_api",
            Self::MobileApp => "proj_mobile_app",
            Self::EcommerceSite => "proj_ecommerce",
            Self::SaasProduct => "proj_saas",
            Self::DataPipeline => "proj_data_pipeline",
            Self::MlModel => "proj_ml_model",
            Self::EnterpriseSoftware => "proj_enterprise",
            Self::OpenSourceFramework => "proj_open_source",
            Self::CryptoProtocol => "proj_crypto",
            Self::GameDev => "proj_gamedev",
            Self::HealthcarePortal => "proj_healthcare",
            Self::LegalDiscovery => "proj_legal",
            Self::FinancialEngine => "proj_financial",
            Self::GovContract => "proj_gov_contract",
            Self::ScientificSim => "proj_scientific",
            Self::AutonomousFleet => "proj_autonomous",
            Self::AgiResearch => "proj_agi_research",
            Self::QuantumCompiler => "proj_quantum_compiler",
            Self::NanotechPrototype => "proj_nanotech",
            Self::HumanoidChassis => "proj_humanoid",
            Self::RoboticsFactory => "proj_robotics_factory",
            Self::ConsciousnessUpload => "proj_consciousness",
            Self::LaunchVehicle => "proj_launch_vehicle",
            Self::OrbitalStation => "proj_orbital_station",
            Self::DeepSpaceProbe => "proj_deep_space",
            Self::AsteroidMining => "proj_asteroid_mining",
            Self::MarsColony => "proj_mars_colony",
            Self::InterplanetaryNet => "proj_interplanetary",
            Self::DysonSwarmSegment => "proj_dyson_segment",
            Self::ComputroniumSlab => "proj_computronium_slab",
        }
    }

    pub fn phase(&self) -> GamePhase {
        match self {
            Self::LandingPage
            | Self::PersonalWebsite
            | Self::SimpleScript
            | Self::CrudApp
            | Self::RestApi
            | Self::MobileApp
            | Self::EcommerceSite
            | Self::SaasProduct
            | Self::DataPipeline
            | Self::MlModel
            | Self::EnterpriseSoftware
            | Self::OpenSourceFramework
            | Self::CryptoProtocol
            | Self::GameDev => GamePhase::Consultancy,
            Self::HealthcarePortal
            | Self::LegalDiscovery
            | Self::FinancialEngine
            | Self::GovContract
            | Self::ScientificSim
            | Self::AutonomousFleet => GamePhase::Industry,
            Self::AgiResearch
            | Self::QuantumCompiler
            | Self::NanotechPrototype
            | Self::HumanoidChassis
            | Self::RoboticsFactory
            | Self::ConsciousnessUpload => GamePhase::PostHuman,
            Self::LaunchVehicle
            | Self::OrbitalStation
            | Self::DeepSpaceProbe
            | Self::AsteroidMining
            | Self::MarsColony
            | Self::InterplanetaryNet => GamePhase::SpaceAge,
            Self::DysonSwarmSegment | Self::ComputroniumSlab => GamePhase::Kardashev,
        }
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

// ---------------------------------------------------------------------------
// Events & Commits
// ---------------------------------------------------------------------------

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
    PhaseTransition,
    MegaProjectUpdate,
    Victory,
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
            Self::PhaseTransition => "##",
            Self::MegaProjectUpdate => "==",
            Self::Victory => "!!",
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

// ---------------------------------------------------------------------------
// Prestige
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Perks
// ---------------------------------------------------------------------------

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
