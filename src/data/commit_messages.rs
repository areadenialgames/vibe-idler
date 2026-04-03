use rand::Rng;

const PREFIXES: &[&str] = &[
    "feat", "fix", "refactor", "chore", "test", "docs", "perf", "style", "ci",
];

const MODULES: &[&str] = &[
    "auth", "api", "db", "ui", "payments", "users", "search", "cache",
    "config", "middleware", "routing", "logging", "metrics", "worker",
    "queue", "storage", "email", "websocket", "graphql", "webhook",
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
];

pub fn random_commit(rng: &mut impl Rng) -> String {
    let prefix = PREFIXES[rng.gen_range(0..PREFIXES.len())];
    let module = MODULES[rng.gen_range(0..MODULES.len())];

    let action = match prefix {
        "feat" => FEAT_ACTIONS[rng.gen_range(0..FEAT_ACTIONS.len())],
        "fix" => FIX_ACTIONS[rng.gen_range(0..FIX_ACTIONS.len())],
        "refactor" => REFACTOR_ACTIONS[rng.gen_range(0..REFACTOR_ACTIONS.len())],
        "chore" | "ci" => CHORE_ACTIONS[rng.gen_range(0..CHORE_ACTIONS.len())],
        "test" => "add unit tests for module",
        "docs" => "update API documentation",
        "perf" => "optimize query performance",
        "style" => "fix formatting and lint issues",
        _ => "update module",
    };

    format!("{}({}): {}", prefix, module, action)
}
