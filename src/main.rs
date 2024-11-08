mod models;
mod handlers;
extern crate dotenv;

use dotenv::dotenv;
// use std::env;
use std::sync::Mutex;
use rusqlite::Connection;
use actix_web::{web, App, HttpServer};

// a wrapper type for thread-safe connection sharing
pub struct DbConnection {
    pub conn: Mutex<Connection>
}

// entry point of application

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   // load env
   dotenv().ok();

   println!("\n🦉 Night Owl Server Starting...");

   // initialize SQLite connection
   println!("📂 Initializing SQLite database connection...");
   let conn = Connection::open("./night-owl.db")
       .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

   // initialize the database
   println!("🔧 Setting up database tables...");
   models::initialize_db(&conn)
       .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
   let connection = web::Data::new(DbConnection {
       conn: Mutex::new(conn)
   });

    let host = "127.0.0.1";
    let port = 8080;

    println!("🚀 Starting server...");
    println!("✨ Server started successfully!");
    println!("🌐 Server is running at: http://{}:{}", host, port);
    println!("📍 Available routes:");
    println!("   GET  http://{}:{}/api/", host, port);
    println!("   POST http://{}:{}/api/add", host, port);
    println!("   POST http://{}:{}/api/get", host, port);
    println!("\n🛑 Press Ctrl+C to stop the server\n");
    
    // starting web server
    HttpServer::new(move || {
        App::new()
            // sharing the SQLite connection across the api handlers
            .app_data(connection.clone())
            .service(
                web::scope("/api")
                    .route("/", web::get().to(handlers::hello_world))
                    .route("/add", web::post().to(handlers::add_log))
                    .route("/get", web::post().to(handlers::get_log))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}