use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub mint: String,
    pub entry_price: f64,
    pub amount: f64,
    pub highest_price: f64,
    pub is_paper: bool,
    pub tp_half_hit: bool,
}

pub struct RiskManager {
    pub trade_amount_sol: f64,
    pub stop_loss_pct: f64,
    pub trailing_buffer_pct: f64,
    pub tp_pct: f64,
}

impl RiskManager {
    pub fn new() -> Self {
        Self {
            trade_amount_sol: 0.05, // Default ~9$ worth of SOL
            stop_loss_pct: 0.50,    // 50%
            trailing_buffer_pct: 0.20, // 20%
            tp_pct: 1.0,            // 100%
        }
    }

    pub fn calculate_exit(&mut self, position: &mut Position, current_price: f64) -> Option<TradeAction> {
        // Update highest price for trailing
        if current_price > position.highest_price {
            position.highest_price = current_price;
        }

        // 1. Check Take Profit 100% (Sell 50% and move SL to BE)
        if !position.tp_half_hit && current_price >= position.entry_price * (1.0 + self.tp_pct) {
            position.tp_half_hit = true;
            return Some(TradeAction::SellHalf);
        }

        // 2. Check Trailing Stop Loss
        // If TP half hit, SL is at Break Even or Trailing
        let current_sl_price = if position.tp_half_hit {
            // After TP, SL is max(Entry, Highest * 0.8)
            let trailing = position.highest_price * (1.0 - self.trailing_buffer_pct);
            if trailing > position.entry_price { trailing } else { position.entry_price }
        } else {
            // Standard SL
            position.entry_price * (1.0 - self.stop_loss_pct)
        };

        if current_price <= current_sl_price {
            return Some(TradeAction::SellAll);
        }

        None
    }
}

pub enum TradeAction {
    SellHalf,
    SellAll,
}
