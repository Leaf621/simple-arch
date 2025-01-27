use std::fmt::Debug;

use uuid::Uuid;

use super::engine::{RefEngine, types::Seller, repository::RepositoryError};

use super::Market;

#[derive(Debug, Clone)]
pub enum SellerError {
    InvalidUsername,
    Repository(RepositoryError),
}

impl Market {
    pub async fn create_seller(&self, username: String) -> Result<SellerContext, SellerError> {
        let username = {
            if username.trim().len() == 0 {
                Err(SellerError::InvalidUsername)
            } else {
                Ok(username)
            }
        }?;
            
        let mut engine = self.engine.borrow().await;
        let seller = Seller {
            id: Uuid::new_v4(),
            username,
            created_at: chrono::Utc::now(),
        };
        let repository = engine.get_repository_mut();
        repository.create_seller(&seller).await.map_err(SellerError::Repository)?;
        Ok(
            SellerContext {
                engine: self.engine.clone(),
                seller,
            }
        )
    }
}

pub struct SellerContext {
    engine: RefEngine,
    seller: Seller,
}

impl Debug for SellerContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.seller.fmt(f)
    }
}

impl SellerContext {
    pub fn get_dto(&self) -> Seller {
        self.seller.clone()
    }

    pub fn get_engine<'a>(&'a self) -> &'a RefEngine {
        &self.engine
    }
}
