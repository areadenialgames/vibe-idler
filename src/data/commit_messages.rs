use crate::game::state::GamePhase;
use rand::Rng;

const PREFIXES: &[&str] = &[
    "feat", "fix", "refactor", "chore", "test", "docs", "perf", "style", "ci",
];

const MODULES: &[&str] = &[
    "auth", "api", "db", "ui", "payments", "users", "search", "cache",
    "config", "middleware", "routing", "logging", "metrics", "worker",
    "queue", "storage", "email", "websocket", "graphql", "webhook",
    "cdn", "i18n", "analytics", "billing", "scheduler", "gateway",
    "proxy", "session", "notifications", "permissions",
];

const FEAT_ACTIONS: &[&str] = &[
    "implement OAuth2 flow",
    "add user dashboard",
    "create REST endpoints",
    "implement rate limiting",
    "add webhook support",
    "implement file uploads",
    "add search functionality",
    "create notification system",
    "implement caching layer",
    "add dark mode support",
    "implement real-time updates",
    "add export to CSV",
    "create admin panel",
    "implement SSO integration",
    "add multi-tenancy support",
    "create analytics dashboard",
    "implement batch processing",
    "add audit logging",
    "create API versioning",
    "implement retry logic",
    "add two-factor auth",
    "implement role-based access",
    "add Stripe integration",
    "create onboarding flow",
    "implement data export pipeline",
    "add custom field support",
    "create automated reports",
    "implement drag-and-drop UI",
    "add real-time collaboration",
    "create plugin architecture",
];

const FIX_ACTIONS: &[&str] = &[
    "resolve N+1 query issue",
    "fix race condition in worker",
    "patch XSS vulnerability",
    "fix memory leak in cache",
    "resolve deadlock in queue",
    "fix timezone handling",
    "patch auth token refresh",
    "fix pagination offset bug",
    "resolve CORS configuration",
    "fix connection pool exhaustion",
    "patch SQL injection vector",
    "fix race in concurrent writes",
    "resolve event ordering bug",
    "fix null pointer in parser",
    "patch session invalidation",
    "fix infinite redirect loop",
    "resolve stale cache invalidation",
    "fix UTF-8 encoding in exports",
    "patch rate limiter bypass",
    "fix WebSocket reconnection logic",
    "resolve duplicate key constraint",
    "fix off-by-one in pagination",
    "patch CSRF token validation",
    "fix leaked database connections",
    "resolve flaky integration test",
];

const REFACTOR_ACTIONS: &[&str] = &[
    "extract service layer",
    "split monolith module",
    "migrate to async/await",
    "normalize database schema",
    "decouple auth middleware",
    "extract shared utilities",
    "simplify query builder",
    "restructure file layout",
    "convert to TypeScript",
    "replace callbacks with promises",
    "consolidate error handling",
    "introduce repository pattern",
    "flatten nested callbacks",
    "extract configuration module",
    "migrate to connection pooling",
];

const CHORE_ACTIONS: &[&str] = &[
    "update dependencies",
    "bump Node.js version",
    "configure ESLint rules",
    "update Docker image",
    "add pre-commit hooks",
    "configure CI pipeline",
    "update README badges",
    "clean up dead code",
    "add .env.example",
    "update license headers",
    "pin dependency versions",
    "add Dependabot config",
    "update GitHub Actions",
    "migrate to pnpm",
    "configure Renovate bot",
];

// Phase 1: Industry
const INDUSTRY_MODULES: &[&str] = &[
    "medical-ai", "legal-nlp", "fintech", "gov-portal", "compliance",
    "patient-records", "court-filings", "trading-algo", "research-db", "hipaa",
    "ehr-sync", "claims-engine", "risk-model", "regulatory", "audit-trail",
    "drug-interaction", "case-law", "portfolio-mgmt", "clearance", "grant-db",
    "diagnostics", "insurance", "tax-engine", "verdict-ai", "clinical-trial",
];

const INDUSTRY_ACTIONS: &[&str] = &[
    "add HIPAA compliance layer",
    "implement medical image classifier",
    "add legal document parser",
    "implement fraud detection model",
    "add government security clearance check",
    "implement real-time vitals dashboard",
    "create automated legal brief generator",
    "add regulatory compliance scanner",
    "implement patient data anonymizer",
    "add financial risk assessment engine",
    "integrate electronic health records",
    "implement drug interaction checker",
    "add case law citation finder",
    "create portfolio rebalancing algorithm",
    "implement insurance claims processor",
    "add multi-jurisdiction tax calculator",
    "create clinical trial matching engine",
    "implement court document OCR pipeline",
    "add real-time market data aggregator",
    "create government audit report generator",
    "implement FHIR-compliant API endpoints",
    "add predictive patient readmission model",
    "create automated contract review system",
    "implement algorithmic trading backtester",
    "add biometric authentication for gov portals",
];

// Phase 2: Post-Human
const POSTHUMAN_PREFIXES: &[&str] = &[
    "feat", "fix", "breakthrough", "evolve", "optimize", "synthesize",
    "transcend", "emerge", "converge", "awaken",
];

const POSTHUMAN_MODULES: &[&str] = &[
    "agi-core", "neural-fabric", "robot-firmware", "quantum-net", "consciousness",
    "self-improve", "ethics-module", "nanoswarm", "qubit-array", "heuristics",
    "synapse-map", "motor-cortex", "vision-stack", "language-center", "empathy-sim",
    "creativity-engine", "dream-state", "memory-palace", "reflex-arc", "ego-kernel",
    "quantum-gate", "entanglement", "decoherence", "superposition", "wave-function",
];

const POSTHUMAN_ACTIONS: &[&str] = &[
    "neural pathway self-optimization complete",
    "quantum error correction at 99.97%",
    "robot motor calibration improved 3x",
    "nanobot swarm achieved consensus",
    "consciousness backup restored successfully",
    "AGI passed Turing test (again)",
    "quantum entanglement stabilized at room temp",
    "humanoid gait now indistinguishable from human",
    "self-modifying code converged on optimal solution",
    "ethical constraint matrix updated",
    "robot learned to make coffee (finally)",
    "quantum compiler reduced decoherence by 40%",
    "synthetic empathy module calibrated",
    "dream-state simulation produced novel theorem",
    "memory consolidation efficiency up 200%",
    "creative problem-solving benchmark exceeded humans",
    "motor learning achieved in 3 iterations vs 10000",
    "language model achieved true understanding (maybe)",
    "nanoswarm self-repair protocol activated",
    "quantum coherence maintained for 47 seconds",
    "ego-kernel boundary conditions stabilized",
    "recursive self-improvement rate: 2.3x per cycle",
    "vision system resolution: 0.01 arc-seconds",
    "reflex arc latency reduced to 0.3ms",
    "synthetic neuron firing rate: 10 GHz",
    "consciousness upload fidelity: 99.999%",
    "robot hand dexterity exceeded surgical precision",
    "quantum teleportation fidelity: 99.8%",
    "AGI discovered new proof technique autonomously",
    "nanobot collective intelligence emerged spontaneously",
];

// Phase 3: Space
const SPACE_PREFIXES: &[&str] = &[
    "launch", "orbit", "deploy", "calibrate", "navigate", "dock",
    "survey", "extract", "transmit", "descend", "rendezvous", "boost",
];

const SPACE_MODULES: &[&str] = &[
    "propulsion", "life-support", "orbital-mech", "mining-drone", "navigation",
    "solar-sail", "habitat-ring", "comm-array", "delta-v", "aerobrake",
    "ion-drive", "reactor-core", "rad-shield", "docking-port", "airlock",
    "greenhouse", "water-recycle", "atmo-gen", "solar-array", "thermal-ctrl",
    "asteroid-scan", "ore-process", "fuel-synth", "trajectory", "telemetry",
    "cargo-bay", "crew-quarters", "med-bay", "lab-module", "power-grid",
];

const SPACE_ACTIONS: &[&str] = &[
    "orbital insertion burn complete",
    "solar panel array deployed successfully",
    "asteroid mining yield up 200%",
    "deep space probe signal acquired",
    "Mars habitat pressurization nominal",
    "interplanetary relay network stable",
    "lunar regolith processing online",
    "zero-g manufacturing module activated",
    "asteroid redirect maneuver calculated",
    "space elevator cable tension optimal",
    "ion drive efficiency at 94%",
    "reactor output stable at 2.4 GW",
    "radiation shielding integrity confirmed",
    "docking sequence completed in 12 seconds",
    "airlock cycle time reduced to 90 seconds",
    "greenhouse crop yield: 400kg/month",
    "water recycling efficiency: 99.7%",
    "atmosphere generator producing 200L O2/hour",
    "thermal control maintaining 22C +/- 0.5",
    "asteroid composition scan: 47% iron, 12% nickel",
    "ore processing throughput: 50 tons/day",
    "fuel synthesis rate: 500L methane/day",
    "trajectory correction: 0.003 m/s delta-v applied",
    "telemetry link stable at 2.4 AU distance",
    "crew rotation schedule optimized",
    "lab module experiment batch 47 complete",
    "power grid load balanced across 12 modules",
    "cargo transfer: 2000kg to surface",
    "med bay: zero incidents this quarter",
    "communication delay compensated: 14.2 minutes",
];

// Phase 4: Kardashev
const KARDASHEV_PREFIXES: &[&str] = &[
    "construct", "align", "calibrate", "harvest", "convert", "integrate",
    "deploy", "activate", "optimize", "expand", "stabilize", "commission",
];

const KARDASHEV_MODULES: &[&str] = &[
    "dyson-panel", "solar-collector", "computronium", "von-neumann",
    "stellar-tap", "mass-driver", "planetary-core", "matter-converter",
    "photon-sail", "energy-grid", "swarm-coord", "beam-focus",
    "heat-sink", "mirror-array", "orbit-keeper", "power-relay",
    "substrate-layer", "logic-mesh", "cooling-system", "data-trunk",
    "gravity-anchor", "tidal-lock", "magnetic-sheath", "plasma-funnel",
    "neutrino-detector", "solar-wind", "corona-shield", "chromosphere-tap",
    "mantle-drill", "core-extractor", "crust-recycler", "ocean-processor",
    "atmo-stripper", "ring-dismantler", "moon-mover", "asteroid-feeder",
];

const KARDASHEV_ACTIONS: &[&str] = &[
    "Dyson panel alignment within 0.001 arc-seconds",
    "solar collector efficiency at 99.8%",
    "computronium substrate integration complete",
    "von Neumann probe self-replicated successfully",
    "stellar energy tap drawing 10^26 watts",
    "planetary mass conversion at 47%",
    "matter-to-computation ratio optimized",
    "photon harvesting grid expanded by 0.1%",
    "Neptune core conversion ahead of schedule",
    "solar system compute network latency: 4 light-minutes",
    "swarm coordination protocol v47.3 deployed",
    "beam focusing accuracy: 99.9997%",
    "heat dissipation nominal at 10^24 watts",
    "mirror array reflecting 0.3% of total solar output",
    "orbital station-keeping adjusted for panel #8,847",
    "power relay bandwidth: 10^22 watts sustained",
    "substrate layer density: 10^18 transistors per cm^3",
    "logic mesh self-test: zero errors in 10^30 operations",
    "cooling system maintaining 3K above theoretical minimum",
    "data trunk throughput: 10^24 bits per second",
    "gravity anchor holding panel cluster at L4",
    "tidal forces compensated across 847 panel segments",
    "magnetic sheath deflecting 99.99% of CME particles",
    "plasma funnel channeling 10^8 kg/s of solar material",
    "solar wind energy capture: additional 10^20 watts",
    "corona shield temperature tolerance: 2 million kelvin",
    "mantle drill reaching 400km depth on target body",
    "core extraction rate: 10^6 tonnes per hour",
    "crust recycling producing 99.7% pure computronium",
    "ocean processing: 10^15 liters converted this cycle",
    "atmosphere stripped and stored for future use",
    "ring material sorted: 10^18 kg ready for conversion",
    "moon repositioned to optimal conversion orbit",
    "asteroid feeder delivering 10^9 kg/day to foundry",
    "new panel commissioned: total output +0.001%",
    "self-repair nanobots maintaining 99.999% uptime",
    "energy surplus detected: allocating to computation",
    "stellar luminosity capture now at 52.7%",
    "computation density: 10^42 FLOPS per solar mass",
    "inter-panel communication: zero packet loss",
];

pub fn random_commit(rng: &mut impl Rng, phase: GamePhase) -> String {
    match phase {
        GamePhase::Consultancy => {
            let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
            let module = MODULES[rng.gen_range(0..MODULES.len())];
            let action = pick_action(rng, prefix);
            format!("{}({}): {}", prefix, module, action)
        }
        GamePhase::Industry => {
            // Mix of normal and industry commits
            if rng.gen_bool(0.5) {
                let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
                let module = INDUSTRY_MODULES[rng.gen_range(0..INDUSTRY_MODULES.len())];
                let action = INDUSTRY_ACTIONS[rng.gen_range(0..INDUSTRY_ACTIONS.len())];
                format!("{}({}): {}", prefix, module, action)
            } else {
                let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
                let module = MODULES[rng.gen_range(0..MODULES.len())];
                let action = pick_action(rng, prefix);
                format!("{}({}): {}", prefix, module, action)
            }
        }
        GamePhase::PostHuman => {
            let prefix = POSTHUMAN_PREFIXES[rng.gen_range(0..POSTHUMAN_PREFIXES.len())];
            let module = POSTHUMAN_MODULES[rng.gen_range(0..POSTHUMAN_MODULES.len())];
            let action = POSTHUMAN_ACTIONS[rng.gen_range(0..POSTHUMAN_ACTIONS.len())];
            format!("{}({}): {}", prefix, module, action)
        }
        GamePhase::SpaceAge => {
            let prefix = SPACE_PREFIXES[rng.gen_range(0..SPACE_PREFIXES.len())];
            let module = SPACE_MODULES[rng.gen_range(0..SPACE_MODULES.len())];
            let action = SPACE_ACTIONS[rng.gen_range(0..SPACE_ACTIONS.len())];
            format!("{}({}): {}", prefix, module, action)
        }
        GamePhase::Kardashev | GamePhase::Victory => {
            let prefix = KARDASHEV_PREFIXES[rng.gen_range(0..KARDASHEV_PREFIXES.len())];
            let module = KARDASHEV_MODULES[rng.gen_range(0..KARDASHEV_MODULES.len())];
            let action = KARDASHEV_ACTIONS[rng.gen_range(0..KARDASHEV_ACTIONS.len())];
            format!("{}({}): {}", prefix, module, action)
        }
    }
}

fn pick_action(rng: &mut impl Rng, prefix: &str) -> &'static str {
    match prefix {
        "feat" => FEAT_ACTIONS[rng.gen_range(0..FEAT_ACTIONS.len())],
        "fix" => FIX_ACTIONS[rng.gen_range(0..FIX_ACTIONS.len())],
        "refactor" => REFACTOR_ACTIONS[rng.gen_range(0..REFACTOR_ACTIONS.len())],
        "chore" | "ci" => CHORE_ACTIONS[rng.gen_range(0..CHORE_ACTIONS.len())],
        "test" => "add unit tests for module",
        "docs" => "update API documentation",
        "perf" => "optimize query performance",
        "style" => "fix formatting and lint issues",
        _ => "update module",
    }
}
