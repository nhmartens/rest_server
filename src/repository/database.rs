use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::client::Client;
use crate::models::project::Project;
use crate::repository::schema::clients;//::dsl::*;
use crate::repository::schema::projects;

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

    // Clients
    pub fn get_clients(&self) -> Vec<Client> {
        clients::table
            .load::<Client>(&mut self.pool.get().unwrap())
            .expect("Error loading all clients")
    }

    pub fn create_client(&self, client: Client) -> Result<Client, Error> {
        let client = Client{
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc().to_string()),
            updated_at: Some(Utc::now().naive_utc().to_string()),
            ..client
        };
        diesel::insert_into(clients::table)
            .values(&client)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating a new client");
        Ok(client)
    }

    pub fn delete_client_by_id(&self, client_id: &str) -> Option<usize> {
        let count = diesel::delete(clients::table.find(client_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting client by id");
        Some(count)
    }

    // Projects
    pub fn get_projects(&self) -> Vec<Project> {
        projects::table
            .load::<Project>(&mut self.pool.get().unwrap())
            .expect("Error loading all projects")
    }

    pub fn create_project(&self, project: Project) -> Result<Project, Error> {
        let project = Project{
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc().to_string()),
            updated_at: Some(Utc::now().naive_utc().to_string()),
            ..project
        };
        diesel::insert_into(projects::table)
            .values(&project)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating a new project");
        Ok(project)
    }

    pub fn delete_project_by_id(&self, project_id: &str) -> Option<usize> {
        let count = diesel::delete(projects::table.find(project_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting project by id");
        Some(count)
    }


}
