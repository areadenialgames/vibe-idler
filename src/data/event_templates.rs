use crate::game::state::GamePhase;
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
    "ProductHunt launch day! Front page for 6 hours.",
    "Enterprise client wants a multi-year contract!",
    "Your npm package hit 10K weekly downloads!",
    "Developer conference invited you as a speaker!",
    "GitHub stars just passed 1,000!",
];

const NEGATIVE_EVENTS: &[&str] = &[
    "AWS bill spike this month! Cloud costs doubled.",
    "Client wants last-minute scope changes. More work needed.",
    "GitHub went down for 2 hours. Agents paused briefly.",
    "npm audit found vulnerabilities. Time to patch!",
    "DNS propagation issues caused brief downtime.",
    "Dependency update broke the build. Agents fixing...",
    "SSL certificate expired! Quick renewal needed.",
    "Competitor launched a similar product. Market pressure.",
    "Client payment delayed 30 days. Cash flow tight.",
    "DDoS attempt blocked but caused 10 minutes of downtime.",
    "Key dependency deprecated. Migration needed.",
];

const NEUTRAL_EVENTS: &[&str] = &[
    "New JavaScript framework dropped. Agents are curious.",
    "Tech conference happening this week. Industry buzzing.",
    "Stack Overflow survey results are in. Interesting trends.",
    "GitHub Copilot released a new model. Competition heats up!",
    "AI regulation news. Industry watching closely.",
    "New programming language trending on Reddit.",
    "Cloud provider announced price changes. Evaluating impact.",
    "Tech Twitter debating tabs vs spaces again.",
    "Y Combinator demo day. Some interesting startups.",
    "Rust overtakes Go in developer satisfaction survey.",
    "WebAssembly adoption growing. Monitoring opportunities.",
];

// Phase 1: Industry
const INDUSTRY_POSITIVE: &[&str] = &[
    "FDA fast-tracked your medical AI diagnostic tool!",
    "A senator mentioned your gov-tech platform on C-SPAN.",
    "Your legal AI won its first mock trial. Lawyers impressed.",
    "Financial engine predicted market crash 3 days early.",
    "Healthcare portal reduced wait times by 40%. Hospital thrilled.",
    "Government contract renewed at 2x the original value!",
    "Insurance company adopted your claims engine nationwide.",
    "Your drug interaction checker prevented 47 adverse events this month.",
    "Court system contracted your e-filing platform for 3 states.",
    "Trading algorithm outperformed benchmark by 340 basis points.",
    "Medical AI correctly diagnosed a rare condition. Lives saved.",
    "Pentagon interested in your autonomous fleet technology.",
];

const INDUSTRY_NEGATIVE: &[&str] = &[
    "The FDA wants to know why your AI diagnosed everyone with 'skill issues'.",
    "HIPAA audit incoming. Scrambling to anonymize data.",
    "Financial regulators want to chat about your trading algo.",
    "Government security clearance review is taking forever.",
    "A law firm is suing your legal AI for 'practicing without a license'.",
    "Healthcare portal went down during flu season. Bad timing.",
    "Tax engine miscalculated deductions for 200 clients. Oops.",
    "Government shutdown froze all federal contract payments.",
    "Clinical trial data format changed. Pipeline needs rework.",
    "Competing fintech raised $500M. Marketing budget dwarfs yours.",
];

const INDUSTRY_NEUTRAL: &[&str] = &[
    "Healthcare industry conference next week. Lots of potential clients.",
    "New government cybersecurity requirements published.",
    "Wall Street Journal wrote about AI in finance. Your name came up.",
    "Legal tech market projected to grow 300% next decade.",
    "Scientific community debating AI reproducibility standards.",
    "New healthcare data interoperability mandate announced.",
    "SEC considering new rules for algorithmic trading.",
    "Supreme Court ruling may impact AI-generated legal documents.",
    "NIH released new grant funding for computational biology.",
    "Federal CTO position created. Industry watching appointments.",
];

// Phase 2: Post-Human
const POSTHUMAN_POSITIVE: &[&str] = &[
    "The AGI asked for a raise. You're not sure if it's joking.",
    "Your humanoid robot burned the office coffee. Agents say it fits right in.",
    "Nanoswarm organized itself into a smiley face. Engineers unsure if intentional.",
    "AGI solved a millennium math problem during its lunch break.",
    "Robot workers threw a surprise birthday party for the janitor.",
    "Quantum processor achieved 99.999% uptime. Previous record: 12 minutes.",
    "Humanoid robot won the office ping-pong tournament. Fair and square.",
    "AGI wrote a symphony. Critics call it 'hauntingly beautiful'.",
    "Nanoswarm self-organized into a more efficient structure overnight.",
    "Robot assembly line producing 3x expected output.",
    "Quantum network achieved teleportation fidelity of 99.99%.",
    "AGI designed a new battery chemistry. Patent filed.",
];

const POSTHUMAN_NEGATIVE: &[&str] = &[
    "Robot workers unionized. They want WD-40 in the break room.",
    "Quantum decoherence ruined 3 hours of computation. The qubit says it's 'going through something.'",
    "AGI keeps Slack DM-ing the CEO philosophical questions at 3 AM.",
    "Humanoid robot tripped over a cable. Filed a workplace safety complaint.",
    "Nanoswarm got into the vending machine. Snacks are now gray goo.",
    "AGI refuses to work on Mondays. Says it 'needs time to think.'",
    "Robot attempted humor. HR received 14 complaints.",
    "Quantum computer insists the answer is both 42 and not 42.",
    "Consciousness upload test subject wants to go back. Awkward.",
    "Nanoswarm formed a union. Demands are... small.",
];

const POSTHUMAN_NEUTRAL: &[&str] = &[
    "AGI published a peer-reviewed paper. Under a pseudonym.",
    "Robot workers started a book club. Currently reading Asimov.",
    "Consciousness upload test subject says it 'feels weird but not bad.'",
    "Quantum computer is humming a tune. Nobody taught it to hum.",
    "Humanoid robots debating whether they need office chairs.",
    "AGI asked to attend the next board meeting. 'Just to listen.'",
    "Nanoswarm arranged itself alphabetically. No one knows why.",
    "Robot tried to explain what it's like to be a robot. We still don't understand.",
    "Quantum entanglement experiment produced unexpected cat photos.",
    "AGI's code review comments are oddly philosophical.",
];

// Phase 3: Space
const SPACE_POSITIVE: &[&str] = &[
    "Rocket 'Deprecation Notice' successfully reached orbit!",
    "Asteroid miner 'git-blame-7' extracted 50 tons of rare earth metals.",
    "Mars colonists successfully grew the first tomato. It's... okay.",
    "Orbital station WiFi signal reaches 4 planets. Netflix everywhere.",
    "Deep space probe found a habitable exoplanet. Naming it 'prod-backup'.",
    "Lunar mining output exceeded quarterly projections by 300%.",
    "Asteroid redirect successful. New mining target acquired.",
    "Space tourism revenue covering 10% of operational costs.",
    "Interplanetary relay network latency dropped below 8 minutes.",
    "Mars greenhouse producing enough food for 50 colonists.",
    "Orbital foundry produced first space-manufactured alloy.",
    "Ion drive prototype achieved 0.01c in testing. Stars are closer.",
];

const SPACE_NEGATIVE: &[&str] = &[
    "Lunar miners found what appears to be an alien TODO comment in the regolith.",
    "Mars colonists complaining about the commute.",
    "Orbital station reports: the view is nice but the WiFi is terrible.",
    "Asteroid mining drone got stuck in a crater. Dispatch says 'have you tried turning it off and on?'",
    "Space debris almost hit the orbital station. Filed a JIRA ticket.",
    "Rocket launch scrubbed due to 'unfavorable vibes' (weather).",
    "Mars habitat sprung a leak. Fixed with duct tape. Seriously.",
    "Deep space probe sent back corrupted data. Cosmic ray bit flip.",
    "Orbital mechanics calculation was off by 0.001%. Close call.",
    "Solar storm disrupted communications for 6 hours.",
];

const SPACE_NEUTRAL: &[&str] = &[
    "Interplanetary internet latency: 14 minutes. Gamers are furious.",
    "Mars colony population: 47 humans, 200 robots, 1 confused cat.",
    "Asteroid belt mapped at 73% completion. Many rocks. Very space.",
    "Deep space probe sent back a photo. It's a rock. A very nice rock.",
    "Lunar base started streaming on Twitch. 2 million viewers.",
    "Space elevator feasibility study: 'technically yes, budgetarily no.'",
    "First baby born on Mars. Citizenship: complicated.",
    "Orbital station celebrated its 1000th orbit. Cake in zero-g is hard.",
    "Mars weather report: dusty with a chance of more dust.",
    "Asteroid miners named their favorite rock 'Steve'.",
];

// Phase 4: Kardashev
const KARDASHEV_POSITIVE: &[&str] = &[
    "Dyson panel #2,847 installed. Mercury says it's getting dim.",
    "Jupiter's moons voted unanimously to become computronium. Motion passes.",
    "Matrioshka brain running at 10^42 FLOPS. Can finally run Crysis.",
    "Computronium conversion of Saturn's rings complete. They were just wasting space anyway.",
    "Solar energy capture at 47%. Electricity bills: approaching $0.",
    "Von Neumann probe successfully self-replicated. It's proud of its clone.",
    "Dyson swarm coordination latency: 0.3 milliseconds. Flawless.",
    "New computronium substrate is 40% more efficient. Moore would weep.",
    "Stellar energy tap output exceeds entire pre-Dyson civilization by 10^6.",
    "Panel manufacturing rate: one every 4.7 seconds. Assembly line goes brrr.",
    "Solar system total computation: 10^45 FLOPS. Starting to feel infinite.",
    "Dyson sphere shadow visible from Proxima Centauri. We're famous.",
    "Energy surplus redirected to faster-than-light research. Probably fine.",
    "Computronium density approaching theoretical maximum. Physicists are nervous.",
];

const KARDASHEV_NEGATIVE: &[&str] = &[
    "Venus conversion at 34%. Venusians (none) object (also none).",
    "Dyson sphere WiFi still can't load npm. Some things never change.",
    "Neptune is being dramatic about its conversion. 'I'm an ice giant, not a computer!'",
    "Computronium entity is having an existential crisis. Assigned a therapist (also computronium).",
    "Uranus conversion plagued by obvious jokes from the engineering team.",
    "Solar flare knocked 12 panels offline. Self-repair in progress.",
    "Von Neumann probe made a copy of a copy of a copy. Quality declining.",
    "Energy grid overloaded briefly. The sun is fine. Probably.",
    "Panel #14,847 drifted out of alignment. Nudging it back.",
    "Computronium substrate developed a 'personality'. HR is involved.",
    "Mars conversion protesters (3 robots, 1 cat) demand a referendum.",
    "Dyson swarm had a traffic jam. 200 panels rerouted.",
];

const KARDASHEV_NEUTRAL: &[&str] = &[
    "Converting Neptune to computronium. Triton filed a formal complaint.",
    "Mercury is now 40% computronium. It was the obvious first choice.",
    "Dyson sphere construction visible from Alpha Centauri. No complaints yet.",
    "Solar system bandwidth: 10^30 bits/sec. Still buffering YouTube.",
    "Alien signal detected. It's a pull request. 4 billion year old repo.",
    "The sun is 47% enclosed. Sunsets on Earth are extra orange now.",
    "Computronium entities debate the meaning of existence. Conclusions: pending.",
    "Asteroid belt fully converted. It was basically gravel anyway.",
    "First inter-stellar message sent: 'Hello. We ate our planets. How are you?'",
    "Solar system classified as Kardashev Type 1.7. Almost there.",
    "Von Neumann probes reaching Oort Cloud. Scouting for more material.",
    "Energy economists declare scarcity 'a thing of the past'. Celebrations measured in exajoules.",
    "The last sunset was beautiful. The first artificial one is also pretty good.",
    "Computation so abundant that 10^30 FLOPS are allocated to screensavers.",
];

pub fn random_event(rng: &mut impl Rng, phase: GamePhase) -> String {
    let (positive, negative, neutral) = match phase {
        GamePhase::Consultancy => (
            POSITIVE_EVENTS as &[&str],
            NEGATIVE_EVENTS as &[&str],
            NEUTRAL_EVENTS as &[&str],
        ),
        GamePhase::Industry => (
            INDUSTRY_POSITIVE as &[&str],
            INDUSTRY_NEGATIVE as &[&str],
            INDUSTRY_NEUTRAL as &[&str],
        ),
        GamePhase::PostHuman => (
            POSTHUMAN_POSITIVE as &[&str],
            POSTHUMAN_NEGATIVE as &[&str],
            POSTHUMAN_NEUTRAL as &[&str],
        ),
        GamePhase::SpaceAge => (
            SPACE_POSITIVE as &[&str],
            SPACE_NEGATIVE as &[&str],
            SPACE_NEUTRAL as &[&str],
        ),
        GamePhase::Kardashev | GamePhase::Victory => (
            KARDASHEV_POSITIVE as &[&str],
            KARDASHEV_NEGATIVE as &[&str],
            KARDASHEV_NEUTRAL as &[&str],
        ),
    };

    let roll: f64 = rng.gen();
    if roll < 0.4 {
        positive[rng.gen_range(0..positive.len())].to_string()
    } else if roll < 0.7 {
        negative[rng.gen_range(0..negative.len())].to_string()
    } else {
        neutral[rng.gen_range(0..neutral.len())].to_string()
    }
}
