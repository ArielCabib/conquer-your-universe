use serde::{Deserialize, Serialize};

use crate::constants::MOVE_INTERVAL_MS;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub settlers: Vec<SettlerState>,
    pub next_settler_id: u64,
    pub settler_min_lifespan_ms: f64,
    pub settler_max_lifespan_ms: f64,
    pub houses: Vec<HouseState>,
    pub next_house_id: u64,
    pub settlers_base_capacity: u64,
    pub houses_base_capacity: u64,
    pub settlers_per_house: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            settlers: Vec::new(),
            next_settler_id: 0,
            settler_min_lifespan_ms: 5_000.0,
            settler_max_lifespan_ms: 10_000.0,
            houses: Vec::new(),
            next_house_id: 0,
            settlers_base_capacity: 10,
            houses_base_capacity: 5,
            settlers_per_house: 10,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum SettlerPhase {
    Alive,
    Fading { started_ms: f64 },
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SettlerState {
    pub id: u64,
    pub anchor_x: f64,
    pub anchor_y: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub move_start_ms: f64,
    pub last_direction_change_ms: f64,
    pub birth_ms: f64,
    pub phase: SettlerPhase,
    #[serde(default)]
    pub lifespan_ms: f64,
}

impl SettlerState {
    pub fn new(id: u64, x: f64, y: f64, now_ms: f64, lifespan_ms: f64) -> Self {
        Self {
            id,
            anchor_x: x,
            anchor_y: y,
            target_x: x,
            target_y: y,
            move_start_ms: now_ms,
            last_direction_change_ms: now_ms - MOVE_INTERVAL_MS,
            birth_ms: now_ms,
            phase: SettlerPhase::Alive,
            lifespan_ms,
        }
    }

    pub fn position_at(&self, now_ms: f64) -> (f64, f64) {
        let elapsed = (now_ms - self.move_start_ms).max(0.0);
        let progress = (elapsed / MOVE_INTERVAL_MS).clamp(0.0, 1.0);
        let eased = crate::ease_out_quad(progress);
        let x = self.anchor_x + (self.target_x - self.anchor_x) * eased;
        let y = self.anchor_y + (self.target_y - self.anchor_y) * eased;
        (x, y)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct HouseState {
    pub id: u64,
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub built_ms: f64,
    #[serde(default = "default_house_last_spawn_ms")]
    pub last_spawn_ms: f64,
}

fn default_house_last_spawn_ms() -> f64 {
    f64::NEG_INFINITY
}

impl HouseState {
    pub fn new(id: u64, x: f64, y: f64, built_ms: f64) -> Self {
        Self {
            id,
            x,
            y,
            built_ms,
            last_spawn_ms: default_house_last_spawn_ms(),
        }
    }
}
