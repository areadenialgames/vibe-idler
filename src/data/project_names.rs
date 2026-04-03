use rand::Rng;
use crate::game::state::ProjectKind;

const CLIENTS: &[&str] = &[
    "TechFlow", "DataNova", "CloudPeak", "NexGen", "ByteForge",
    "PixelWave", "CodeVault", "QuantumBit", "SynapseAI", "AeroStack",
    "CyberPulse", "NetSphere", "DevStream", "AlphaNode", "CoreLogic",
    "ZenithLabs", "PrimeSoft", "VectorOps", "OmniTech", "FusionDev",
    "ClearPath", "BluePrint", "IronMesh", "SwiftScale", "TrueNorth",
];

const WEB_PROJECTS: &[&str] = &[
    "Portfolio Pro", "LaunchPad", "BizSite", "ShowCase", "QuickLand",
    "ClickFunnel", "LeadGen Pro", "SplashPage", "HeroSite", "ConvertX",
];

const APP_PROJECTS: &[&str] = &[
    "TaskMaster", "InvoiceBot", "InventoryX", "CRM Lite", "TimerApp",
    "ExpenseTrack", "HRPortal", "BookingWiz", "FormBuilder", "ChatFlow",
];

const SAAS_PROJECTS: &[&str] = &[
    "AnalytiQ", "DeployBot", "MonitorPro", "PipelineX", "DataSync",
    "APIGateway", "LogStream", "MetricHub", "AlertFlow", "StatusPage",
];

const SCRIPT_PROJECTS: &[&str] = &[
    "csv-parser", "log-rotator", "backup-tool", "migration-script",
    "data-scraper", "report-gen", "auto-tagger", "link-checker",
];

pub fn random_client(rng: &mut impl Rng) -> &'static str {
    CLIENTS[rng.gen_range(0..CLIENTS.len())]
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
        _ => {
            APP_PROJECTS[rng.gen_range(0..APP_PROJECTS.len())]
        }
    }
}
