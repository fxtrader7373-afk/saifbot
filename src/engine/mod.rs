pub mod scoring;
pub mod monitor;
pub mod filters;

pub struct IntelligenceEngine {
    pub scoring: scoring::ScoringEngine,
}

impl IntelligenceEngine {
    pub fn new() -> Self {
        Self {
            scoring: scoring::ScoringEngine::new(),
        }
    }
}
