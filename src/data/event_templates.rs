use rand::Rng;

const POSITIVE_EVENTS: &[&str] = &[
    "Your landing page hit Hacker News! +$500 bonus",
    "A tech influencer tweeted about your app! Brand awareness up!",
    "Client left a 5-star review. Reputation growing!",
    "Open source contribution went viral!",
    "Agent discovered a performance optimization trick!",
    "Referral client incoming - word of mouth is working!",
    "Your SaaS was featured in a newsletter!",
    "Stack Overflow answer drove traffic to your project!",
    "Tech blog featured your API design patterns!",
];

const NEGATIVE_EVENTS: &[&str] = &[
    "AWS bill spike this month! Cloud costs doubled.",
    "Client wants last-minute scope changes. More work needed.",
    "GitHub went down for 2 hours. Agents paused briefly.",
    "npm audit found vulnerabilities. Time to patch!",
    "DNS propagation issues caused brief downtime.",
    "Dependency update broke the build. Agents fixing...",
    "SSL certificate expired! Quick renewal needed.",
];

const NEUTRAL_EVENTS: &[&str] = &[
    "New JavaScript framework dropped. Agents are curious.",
    "Tech conference happening this week. Industry buzzing.",
    "Stack Overflow survey results are in. Interesting trends.",
    "GitHub Copilot released a new model. Competition heats up!",
    "AI regulation news. Industry watching closely.",
    "New programming language trending on Reddit.",
    "Cloud provider announced price changes. Evaluating impact.",
];

pub fn random_event(rng: &mut impl Rng) -> String {
    let roll: f64 = rng.gen();
    if roll < 0.4 {
        POSITIVE_EVENTS[rng.gen_range(0..POSITIVE_EVENTS.len())].to_string()
    } else if roll < 0.7 {
        NEGATIVE_EVENTS[rng.gen_range(0..NEGATIVE_EVENTS.len())].to_string()
    } else {
        NEUTRAL_EVENTS[rng.gen_range(0..NEUTRAL_EVENTS.len())].to_string()
    }
}
