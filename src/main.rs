use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool};
use serde::{Serialize};
mod service;
use service::{GreetingService, Info};


#[derive(Serialize)]
struct Message {
    content: String,
}

async fn greet(
    info: web::Json<Info>,
    service: web::Data<GreetingService>,    
) -> impl Responder {
    match service.greet(info.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn hello(service: web::Data<GreetingService>) -> impl Responder {
    match service.welcome().await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn index() -> impl Responder {
    let message = Message {
        content: String::from("Hello, Actix Web with JSON!"),
    };
    web::Json(message)
}

async fn list_names(service: web::Data<GreetingService>) -> impl Responder {
     match service.list_names().await {
        Ok(names) => HttpResponse::Ok().json(names),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
     }
}

async fn name( 
    path: web::Path<i32>,
    service: web::Data<GreetingService>,
) -> impl Responder {
    let id = path.into_inner();
    match service.name_by_id(id).await {
        Ok(Some(greeting)) => HttpResponse::Ok().json(greeting),
        Ok(None) => HttpResponse::NotFound().body(format!("No greeting found with id {}", id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite:greetings.db";
    if !sqlx::Sqlite::database_exists(db_url).await.unwrap_or(false) {
        match sqlx::Sqlite::create_database(db_url).await {
            Ok(()) => println!("Database created successfully"),
            Err(e) => {
                eprintln!("Failed to create database: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "DB creation failed"));
            }
        }
    }
    
    let db_pool = match SqlitePool::connect(db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "DB connection failed"));
        }
    };

    sqlx::query("CREATE TABLE IF NOT EXISTS greetings (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
        .execute(&db_pool)
        .await
        .unwrap();
    
    let greeting_service = GreetingService::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(greeting_service.clone()))
            .route("/", web::get().to(index))
            .route("/greet", web::post().to(greet))
            .route("/hello", web::get().to(hello))
            .route("/names", web::get().to(list_names))
            .route("/names/{id}", web::get().to(name))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
