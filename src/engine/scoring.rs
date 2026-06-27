use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetrics {
    pub mint: String,
    pub dev_wallet: String,
    pub has_socials: bool,
    pub holder_concentration: f64,
    pub is_bundled: bool,
    pub market_cap: f64,
}

pub struct ScoringEngine {
    pub threshold: u8,
}

impl ScoringEngine {
    pub fn new() -> Self {
        Self { threshold: 80 }
    }

    pub fn calculate_score(&self, metrics: &TokenMetrics) -> u8 {
        let mut score: i32 = 50; // Base score

        // Socials check
        if metrics.has_socials {
            score += 15;
        } else {
            score -= 10;
        }

        // Concentration check (assuming % of top 10 holders)
        if metrics.holder_concentration < 20.0 {
            score += 10;
        } else if metrics.holder_concentration > 50.0 {
            score -= 30;
        }

        // Bundled buy check
        if metrics.is_bundled {
            score -= 40;
        } else {
            score += 5;
        }

        // Clamp score between 0 and 100
        score.clamp(0, 100) as u8
    }

    pub fn should_buy(&self, score: u8) -> bool {
        score >= self.threshold
    }
}
