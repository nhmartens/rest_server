use actix_web::{web, get, post, delete, HttpResponse};
use crate::{models::client::Client, models::project::Project, repository::database::Database};

// Clients
#[get("/clients")]
pub async fn get_clients(db: web::Data<Database>) -> HttpResponse {
    let clients = db.get_clients();
    HttpResponse::Ok().json(clients)
}

#[post("/clients")]
pub async fn create_client(db: web::Data<Database>, new_client: web::Json<Client>) -> HttpResponse {
    let client = db.create_client(new_client.into_inner());
    match client {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/clients/{id}")]
pub async fn delete_client_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let client = db.delete_client_by_id(&id);
    match client {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Client not found"),
    }
}

// Projects
#[get("/projects")]
pub async fn get_projects(db: web::Data<Database>) -> HttpResponse {
    let projects = db.get_projects();
    HttpResponse::Ok().json(projects)
}

#[post("/projects")]
pub async fn create_project(db: web::Data<Database>, new_project: web::Json<Project>) -> HttpResponse {
    let project = db.create_project(new_project.into_inner());
    match project {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/projects/{id}")]
pub async fn delete_project_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let project = db.delete_project_by_id(&id);
    match project {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Project not found"),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Clients
            .service(create_client)
            .service(get_clients)
            .service(delete_client_by_id)
            // Projects
            .service(get_projects)
            .service(create_project)
            .service(delete_project_by_id)

    );
}