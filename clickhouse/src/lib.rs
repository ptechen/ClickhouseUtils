use client::prelude::{Connection};
use async_trait::async_trait;

#[async_trait]
pub trait Insert {
    async fn insert(&self, mut connection: Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait Query<T> {
    async fn query(&self, mut connection: Connection, sql: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}