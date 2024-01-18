use actix_web::{web, get, post, delete, HttpResponse};
use crate::{models::client::Client, repository::database::Database};


#[post("/clients")]
pub async fn create_client(db: web::Data<Database>, new_client: web::Json<Client>) -> HttpResponse {
    let client = db.create_client(new_client.into_inner());
    match client {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/clients")]
pub async fn get_clients(db: web::Data<Database>) -> HttpResponse {
    let clients = db.get_clients();
    HttpResponse::Ok().json(clients)
}

#[delete("/clients/{id}")]
pub async fn delete_client_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let client = db.delete_client_by_id(&id);
    match client {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Client not found"),
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_client)
            .service(get_clients)
            .service(delete_client_by_id)
    );
}