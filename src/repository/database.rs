use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::client::Client;
use crate::repository::schema::clients::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_client(&self, client: Client) -> Result<Client, Error> {
        let client = Client{
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc().to_string()),
            updated_at: Some(Utc::now().naive_utc().to_string()),
            ..client
        };
        diesel::insert_into(clients)
            .values(&client)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating a new client");
        Ok(client)
    }

    pub fn get_clients(&self) -> Vec<Client> {
        clients
            .load::<Client>(&mut self.pool.get().unwrap())
            .expect("Error loading all clients")
    }

    pub fn delete_client_by_id(&self, client_id: &str) -> Option<usize> {
        let count = diesel::delete(clients.find(client_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting client by id");
        Some(count)
    }
}
