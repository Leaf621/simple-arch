use async_trait::async_trait;
use uuid::Uuid;

use super::types::{Good, Seller};

#[derive(Debug, Clone)]
pub enum RepositoryError {
    InternalError,
}

#[async_trait]
pub trait Repository {
    async fn create_seller(&mut self, seller: &Seller) -> Result<(), RepositoryError>;
    async fn get_seller(&self, id: Uuid) -> Result<Option<Seller>, RepositoryError>;

    async fn create_good(&mut self, good: &Good) -> Result<(), RepositoryError>;
    async fn get_good(&self, id: Uuid) -> Result<Option<Good>, RepositoryError>;
    async fn delete_good(&mut self, id: Uuid) -> Result<(), RepositoryError>;
}

pub struct RepositoryImpl {
    sellers: Vec<Seller>,
    goods: Vec<Good>,
}

impl RepositoryImpl {
    pub fn new() -> Self {
        Self {
            sellers: Vec::new(),
            goods: Vec::new(),
        }
    }
}

#[async_trait]
impl Repository for RepositoryImpl {
    async fn create_seller(&mut self, seller: &Seller) -> Result<(), RepositoryError> {
        self.sellers.push(seller.clone());
        Ok(())
    }

    async fn get_seller(&self, id: Uuid) -> Result<Option<Seller>, RepositoryError> {
        Ok(
            self.sellers
                .iter()
                .find(|seller| seller.id == id)
                .cloned(),
        )
    }

    async fn create_good(&mut self, good: &Good) -> Result<(), RepositoryError> {
        self.goods.push(good.clone());
        Ok(())
    }

    async fn get_good(&self, id: Uuid) -> Result<Option<Good>, RepositoryError> {
        Ok(
            self.goods
                .iter()
                .find(|good| good.id == id)
                .cloned(),
        )
    }

    async fn delete_good(&mut self, id: Uuid) -> Result<(), RepositoryError> {
        self.goods.retain(|good| good.id != id);
        Ok(())
    }
}
