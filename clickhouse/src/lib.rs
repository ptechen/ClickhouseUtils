use client::prelude::{Connection};

pub trait Insert {
    fn insert(&self, mut connection: Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait Query<T> {
    fn query(&self, mut connection: Connection, sql: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>;
}