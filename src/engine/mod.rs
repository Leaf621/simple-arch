use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};

pub mod repository;
pub mod types;

pub struct Engine {
    repository: Box<dyn repository::Repository>,
}

impl Engine {
    pub fn new(repository: Box<dyn repository::Repository>) -> Self {
        Engine { repository }
    }

    pub fn get_repository(&self) -> &dyn repository::Repository {
        self.repository.as_ref()
    }

    pub fn get_repository_mut(&mut self) -> &mut dyn repository::Repository {
        self.repository.as_mut()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            repository: Box::new(repository::RepositoryImpl::new()),
        }
    }
}

#[derive(Clone)]
pub struct RefEngine {
    engine: Arc<Mutex<Engine>>,
}

impl RefEngine {
    pub fn new(engine: Engine) -> Self {
        RefEngine {
            engine: Arc::new(Mutex::new(engine)),
        }
    }

    pub async fn borrow(&self) -> MutexGuard<Engine> {
        self.engine.lock().await
    }
}
