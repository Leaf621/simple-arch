use std::fmt::Debug;

use uuid::Uuid;

use crate::seller::SellerContext;

use super::{engine::{types::{Good, Seller}, RefEngine, repository::RepositoryError}, Market};

#[derive(Debug, Clone)]
pub enum GoodError {
    Repository(RepositoryError),
    NotEnoughPermissions,
}

impl Market {
    pub async fn get_good(&self, id: Uuid) -> Result<Option<GoodContext>, GoodError> {
        let engine = self.engine.borrow().await;
        let repository = engine.get_repository();
        let good = repository.get_good(id).await.map_err(GoodError::Repository)?;
        Ok(good.map(|good| GoodContext::new(self.engine.clone(), good, None)))
    }
}

pub struct GoodContext {
    engine: RefEngine,
    good: Good,
    viewer: Option<Seller>,
}

impl GoodContext {
    pub fn new(engine: RefEngine, good: Good, viewer: Option<Seller>) -> Self {
        GoodContext { engine, good, viewer }
    }

    pub fn get_dto(&self) -> Good {
        self.good.clone()
    }

    pub async fn delete_good(self) -> Result<(), GoodError> {
        if self.viewer.is_none() {
            return Err(GoodError::NotEnoughPermissions);
        }
        if self.viewer.as_ref().unwrap().id != self.good.seller_id {
            return Err(GoodError::NotEnoughPermissions);
        }
        let mut engine = self.engine.borrow().await;
        let repository = engine.get_repository_mut();
        repository.delete_good(self.good.id).await.map_err(GoodError::Repository)
    }
}

impl Debug for GoodContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoodContext").field("good", &self.good).field("viewer", &self.viewer).finish()
    }
}

impl SellerContext {
    pub async fn create_good(&self, name: String, price: u64) -> Result<GoodContext, GoodError> {
        let seller = self.get_dto();
        let mut engine = self.get_engine().borrow().await;
        let good = Good {
            id: Uuid::new_v4(),
            name,
            price,
            seller_id: seller.id.clone(),
            created_at: chrono::Utc::now(),
        };
        let repository = engine.get_repository_mut();
        repository.create_good(&good).await.map_err(GoodError::Repository)?;
        Ok(
            GoodContext::new(self.get_engine().clone(), good, Some(seller))
        )
    }

    pub async fn get_good(&self, id: Uuid) -> Result<Option<GoodContext>, GoodError> {
        let engine = self.get_engine().borrow().await;
        let repository = engine.get_repository();
        Ok(
            repository.get_good(id).await.map_err(GoodError::Repository)?
                .map(|good| GoodContext::new(self.get_engine().clone(), good, Some(self.get_dto())))
        )
    }
}
