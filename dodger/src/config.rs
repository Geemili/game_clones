use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DodgerConfig {
    /// Configuration of the arena
    pub arena: ArenaConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 100.0,
        }
    }
}
