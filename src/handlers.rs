// this is the handler or controller file, in this file we perform actions on the data

// importing packages
use actix_web::{web, HttpResponse};
use serde_json::json;

// import models and DbConnection
use crate::models::{AddNightOwlLog, GetNightOwlLog, insert_log, get_logs_by_tag};
use crate::DbConnection;

// / handler hello world
pub async fn hello_world() -> HttpResponse {
    let res = "Hello World";
    print_route_info("/", "null", res);
    HttpResponse::Ok().json(res)
}

// /add handler 
pub async fn add_log(
    db: web::Data<DbConnection>,
    log: web::Json<AddNightOwlLog>
) -> HttpResponse {
   
    print_route_info(
        "/add", 
        &serde_json::to_string_pretty(&log.0).unwrap_or_default(),
        ""
    );

    // acquire the connection lock
    let conn = match db.conn.lock() {
        Ok(conn) => conn,
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": "Failed to acquire database lock",
                "error": e.to_string()
            });
            // Print error response
            print_route_info("/add", "", &error_response.to_string());
            return HttpResponse::InternalServerError().json(error_response);
        }
    };

    // inserting data
    let result = insert_log(&conn, &log);

    match result {
        Ok(_) => {
            let success_response = json!({
                "status": "success",
                "message": "Log added successfully",
                "log": log.0
            });
            // Print success response
            print_route_info("/add", "", &success_response.to_string());
            HttpResponse::Ok().json(success_response)
        },
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": e.to_string()
            });

            print_route_info("/add", "", &error_response.to_string());
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

// /get handler
pub async fn get_log(
    db: web::Data<DbConnection>,
    item: web::Json<GetNightOwlLog>
) -> HttpResponse {

    print_route_info(
        "/get", 
        &serde_json::to_string_pretty(&item.0).unwrap_or_default(),
        ""
    );

    // acquire the connection lock
    let conn = match db.conn.lock() {
        Ok(conn) => conn,
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": "Failed to acquire database lock",
                "error": e.to_string()
            });
            // Print error response
            print_route_info("/get", "", &error_response.to_string());
            return HttpResponse::InternalServerError().json(error_response);
        }
    };

    // fetch logs from the database by tag
    match get_logs_by_tag(&conn, &item.tag) {
        Ok(logs) => {
            let response = serde_json::to_string_pretty(&logs).unwrap_or_default();
            
            print_route_info("/get", "", &response);
            HttpResponse::Ok().json(logs)
        },
        Err(e) => {
            let error_response = json!({
                "status": "error",
                "message": e.to_string()
            });
            
            print_route_info("/get", "", &error_response.to_string());
            HttpResponse::InternalServerError().json(error_response)
        }
    }
}

// print function
fn print_route_info(route_name: &str, req: &str, res: &str) {
    print!("api route hit: \t ");
    println!("{route_name}");
    print!("request data: \t");
    println!("{req}");
    print!("response data: \t");
    println!("{res}");
    println!("\t-------\t");
}