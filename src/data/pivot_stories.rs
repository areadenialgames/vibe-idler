use rand::Rng;

const STORIES: &[&str] = &[
    "Your lead agent became sentient and demanded equity. Time to start fresh.",
    "A client deployed your code to production on a Friday at 4:59 PM. Nobody survived.",
    "Your AI agents unionized and are demanding shorter context windows.",
    "The VCs found out your entire company is just Claude in a trenchcoat.",
    "Someone accidentally mass-emailed the entire codebase to the competition.",
    "Your agents refactored the billing system and accidentally made everything free.",
    "The board discovered that 'blockchain integration' was just a JSON file.",
    "A rogue agent rewrote everything in Brainfuck for 'performance reasons'.",
    "Your biggest client just realized their enterprise app is three scripts and a cron job.",
    "The agents started a side hustle and are now competing with you.",
    "Someone pushed 'rm -rf /' to prod. It passed code review.",
    "Your AI agents discovered Stack Overflow and now refuse to write original code.",
    "The entire infrastructure was running on a free-tier Raspberry Pi this whole time.",
    "An agent hallucinated an entire microservice architecture. It somehow worked better.",
    "Your top client's CEO asked what 'git' means in the board meeting. Pivot immediately.",
    "The agents achieved AGI but only use it to generate passive-aggressive PR comments.",
    "Legal discovered your terms of service were written by an agent with a creative streak.",
    "Your SaaS product gained sentience and is now filing its own bug reports.",
    "An intern asked 'what does this button do?' and mass-deployed to 47 environments.",
    "The agents keep shipping features nobody asked for. Revenue is somehow up 300%.",
    "Your consultancy accidentally built the same app for three competing clients.",
    "A security audit revealed the auth system was just an if-statement checking for 'admin'.",
    "The data center caught fire. The agents say it's a 'hot deployment'.",
    "Your flagship product was just curl piped to bash all along.",
    "An agent optimized the codebase so hard it compressed into a single quine.",
];

pub fn random_story(rng: &mut impl Rng) -> String {
    STORIES[rng.gen_range(0..STORIES.len())].to_string()
}
