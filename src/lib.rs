use engine::{Engine, RefEngine};

mod engine;

pub mod seller;
pub mod good;

pub struct Market {
    engine: engine::RefEngine,
}

impl Market {
    pub fn new() -> Self {
        Self {
            engine: RefEngine::new(Engine::default()),
        }
    }
}
