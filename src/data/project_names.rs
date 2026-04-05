use crate::game::state::{GamePhase, ProjectKind};
use rand::Rng;

const CLIENTS: &[&str] = &[
    "TechFlow", "DataNova", "CloudPeak", "NexGen", "ByteForge",
    "PixelWave", "CodeVault", "QuantumBit", "SynapseAI", "AeroStack",
    "CyberPulse", "NetSphere", "DevStream", "AlphaNode", "CoreLogic",
    "ZenithLabs", "PrimeSoft", "VectorOps", "OmniTech", "FusionDev",
    "ClearPath", "BluePrint", "IronMesh", "SwiftScale", "TrueNorth",
    "HexaCore", "VoltStack", "RunLoop", "NebulaIO", "ShardDB",
    "PinPoint", "GridLock", "ArcLight", "DeepRoot", "FluxData",
    "SkyBridge", "WarpDev", "EdgeNode", "PulseNet", "TidalWave",
    "NanoStack", "BrightBit", "SparkLabs", "RiftCloud", "DawnTech",
];

const INDUSTRY_CLIENTS: &[&str] = &[
    "MedVault", "LexiCorp", "QuantFin", "FedSecure", "BioGenesis",
    "CureAI", "JurisBot", "WealthOS", "GovTech", "LabStream",
    "HealthGrid", "LegalMind", "FinCore", "CivicNet", "PharmaSys",
    "MedixAI", "CourthouseIO", "BankChain", "ClearanceHub", "GenomePro",
    "PulseHealth", "LawGraph", "CapitalSync", "SecurGov", "TrialForge",
    "VitalCore", "ClaimBot", "AuditPro", "RegShield", "LabNexus",
    "PatientPath", "LegalEase", "HedgeMind", "TaxGraph", "ClinicalAI",
];

const POSTHUMAN_CLIENTS: &[&str] = &[
    "SingularityLabs", "Prometheus Corp", "Titan Dynamics",
    "Aether Systems", "NovaMind", "Ouroboros AI", "Tesseract Heavy",
    "Infinity Forge", "Eclipse Engineering", "Omega Research",
    "Axiom Neural", "Pantheon Labs", "Cognitum", "Atlas Synthetic",
    "Chrysalis Tech", "Archon Dynamics", "Zenith Minds", "Golem Works",
    "Daedalus Corp", "Hyperion Systems", "Elysium Labs", "Nexus Prime",
    "Babel Industries", "Icarus Dynamics", "Minerva AI",
    "Phoenix Robotics", "Chimera Labs", "Oracle Synthetic",
    "Rapture Engineering", "Transcend Corp",
];

const SPACE_CLIENTS: &[&str] = &[
    "Astral Mining Co", "Orbital Dynamics", "DeepVoid Exploration",
    "Cosmos Foundry", "Stellar Reach", "Lagrange Industries",
    "Perihelion Corp", "Aphelion Systems", "Void Architects",
    "Kuiper Belt Ltd", "Sol Enterprises", "Oort Cloud Mining",
    "Heliosphere Inc", "Red Horizon", "Blue Origin Mk2",
    "Ceres Collective", "Phobos Works", "Ganymede Heavy",
    "Callisto Dynamics", "Enceladus Labs", "Titan Extractors",
    "Pluto Logistics", "Eris Industries", "Sedna Deep",
    "Proxima Reach", "Vesta Mining", "Io Foundries",
    "Europa Aquatics", "Triton Systems", "Charon Outpost",
    "Olympus Station", "Tycho Manufacturing", "Kepler Transit",
    "Hubble Analytics", "Armstrong Base", "Aldrin Corp",
];

const WEB_PROJECTS: &[&str] = &[
    "Portfolio Pro", "LaunchPad", "BizSite", "ShowCase", "QuickLand",
    "ClickFunnel", "LeadGen Pro", "SplashPage", "HeroSite", "ConvertX",
    "PageCraft", "SiteForge", "WebPulse", "PixelDrop", "FormFlow",
    "LinkTree Pro", "BrandKit", "DesignHub", "ThemeForge", "StyleGrid",
];

const APP_PROJECTS: &[&str] = &[
    "TaskMaster", "InvoiceBot", "InventoryX", "CRM Lite", "TimerApp",
    "ExpenseTrack", "HRPortal", "BookingWiz", "FormBuilder", "ChatFlow",
    "ProjectPulse", "DocuSign Lite", "CalendarSync", "NoteVault", "TeamChat",
    "BudgetBuddy", "ClientHub", "TicketDesk", "PollMaker", "SurveyPro",
    "FileShare", "MeetingBot", "KanbanFlow", "SprintBoard", "CodeReview",
];

const SAAS_PROJECTS: &[&str] = &[
    "AnalytiQ", "DeployBot", "MonitorPro", "PipelineX", "DataSync",
    "APIGateway", "LogStream", "MetricHub", "AlertFlow", "StatusPage",
    "CloudWatch", "InfraMap", "SecureVault", "IdentityOS", "PayFlow",
    "TenantPro", "WebhookHub", "QueueMaster", "CachePrime", "SchemaSync",
];

const SCRIPT_PROJECTS: &[&str] = &[
    "csv-parser", "log-rotator", "backup-tool", "migration-script",
    "data-scraper", "report-gen", "auto-tagger", "link-checker",
    "env-sync", "db-seeder", "cron-manager", "test-runner",
    "deploy-hook", "config-gen", "cert-renewer", "health-checker",
];

const INDUSTRY_PROJECTS: &[&str] = &[
    "MediScan", "CaseEngine", "TradeSim", "SecureGov", "LabOS",
    "VitalTrack", "CompliBot", "RiskCalc", "DrugFinder", "TaxFlow",
    "PatientLink", "CourtSync", "AlgoTrade", "ClearanceOS", "TrialDB",
    "ClaimFlow", "AuditTrail", "RegTech", "InsureBot", "GrantWriter",
    "PharmTrack", "LegalPad", "WealthView", "PolicyEngine", "BioMarker",
    "DiagnostiQ", "JuryPool", "LoanCalc", "ComplianceAI", "ResearchDB",
];

const POSTHUMAN_PROJECTS: &[&str] = &[
    "Prometheus", "Athena Core", "Nexus Mind", "Qubit Prime",
    "NanoForge", "SynthLife", "OmegaNet", "Cortex", "Singularity",
    "Archon", "Golem OS", "Titan Brain", "Neural Loom", "Chrysalis",
    "Axiom Core", "Basilisk", "Daemon Net", "Echo Mind", "Flux Engine",
    "Genesis Core", "Hivemind OS", "Icarus Drive", "Janus Gate",
    "Kairos Engine", "Labyrinth", "Morpheus", "Nyx Protocol",
    "Olympus OS", "Pandora Box", "Quantum Lattice",
];

const SPACE_PROJECTS: &[&str] = &[
    "Horizon", "Artemis", "Helios Array", "Vanguard", "Nova Drive",
    "Sol Harvester", "Titan Base", "Deep Survey", "Orbit Ring",
    "Stellar Forge", "Void Walker", "Comet Chaser", "Nebula Gate",
    "Pulsar Relay", "Quasar Tap", "Magnetar Shield", "Red Shift",
    "Dark Matter Probe", "Gravity Lens", "Warp Beacon",
    "Ion Sweep", "Photon Sail", "Plasma Drill", "Fusion Core",
    "Antimatter Cell", "Dyson Lattice", "Oort Sentinel",
    "Kuiper Scanner", "Belt Miner", "Ring Forge",
    "Cryogenic Bay", "Habitat Alpha", "Greenhouse Mars",
    "Terraform Engine", "Colony Pod", "Starport Delta",
    "Relay Gamma", "Dock Epsilon", "Fuel Depot Zeta",
    "Observatory Theta", "Lab Module Iota", "Foundry Kappa",
];

pub fn random_client(rng: &mut impl Rng, phase: GamePhase) -> &'static str {
    let pool = match phase {
        GamePhase::Consultancy => CLIENTS,
        GamePhase::Industry => INDUSTRY_CLIENTS,
        GamePhase::PostHuman => POSTHUMAN_CLIENTS,
        GamePhase::SpaceAge | GamePhase::Kardashev | GamePhase::Victory => SPACE_CLIENTS,
    };
    pool[rng.gen_range(0..pool.len())]
}

pub fn random_project(rng: &mut impl Rng, kind: ProjectKind) -> &'static str {
    match kind {
        ProjectKind::LandingPage | ProjectKind::PersonalWebsite => {
            WEB_PROJECTS[rng.gen_range(0..WEB_PROJECTS.len())]
        }
        ProjectKind::SimpleScript | ProjectKind::DataPipeline => {
            SCRIPT_PROJECTS[rng.gen_range(0..SCRIPT_PROJECTS.len())]
        }
        ProjectKind::SaasProduct | ProjectKind::EnterpriseSoftware | ProjectKind::MlModel => {
            SAAS_PROJECTS[rng.gen_range(0..SAAS_PROJECTS.len())]
        }
        // Phase 1
        ProjectKind::HealthcarePortal
        | ProjectKind::LegalDiscovery
        | ProjectKind::FinancialEngine
        | ProjectKind::GovContract
        | ProjectKind::ScientificSim
        | ProjectKind::AutonomousFleet => {
            INDUSTRY_PROJECTS[rng.gen_range(0..INDUSTRY_PROJECTS.len())]
        }
        // Phase 2
        ProjectKind::AgiResearch
        | ProjectKind::QuantumCompiler
        | ProjectKind::NanotechPrototype
        | ProjectKind::HumanoidChassis
        | ProjectKind::RoboticsFactory
        | ProjectKind::ConsciousnessUpload => {
            POSTHUMAN_PROJECTS[rng.gen_range(0..POSTHUMAN_PROJECTS.len())]
        }
        // Phase 3 & 4
        ProjectKind::LaunchVehicle
        | ProjectKind::OrbitalStation
        | ProjectKind::DeepSpaceProbe
        | ProjectKind::AsteroidMining
        | ProjectKind::MarsColony
        | ProjectKind::InterplanetaryNet
        | ProjectKind::DysonSwarmSegment
        | ProjectKind::ComputroniumSlab => SPACE_PROJECTS[rng.gen_range(0..SPACE_PROJECTS.len())],
        _ => APP_PROJECTS[rng.gen_range(0..APP_PROJECTS.len())],
    }
}
