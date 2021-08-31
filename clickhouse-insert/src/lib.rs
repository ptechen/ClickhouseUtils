use client::prelude::Connection;

pub trait Clickhouse {
    fn insert(&self, mut connection: Connection, table_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}