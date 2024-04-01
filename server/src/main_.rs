use actix_web::get;
use actix_web::rt::time;
use actix_web::web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use rocket::http::hyper::server::conn::{self, Connection};
use rocket::response::status;
use serde::{Deserialize, Serialize}; // Added import for Serialize
use std::error::Error; // Added import for Error
use std::fs; // Added import for fs
use std::net::TcpListener;
use tokio_postgres::{Client, NoTls}; // Added import for web // Added import for RegisterRequest // Added import for DateTime and Utc

// Add module declaration for models

#[derive(Debug, Serialize)]
struct MaintenanceData {
    department: String,
    tasks: Vec<String>,
    time: String,
    date: String,
    status: String,
    description: String,
    assigned_to: String,
    assigned_by: String,
    priority: String,
}

#[derive(Debug, Serialize)]
pub struct Authority {
    pub username: String,
    pub password: String,
    pub status: String,
    pub login_time: String,
    pub logout_time: String,
    pub Admin: bool,
}




pub async fn main_(
    user_authority: &Authority,
    tcp_authorized: &TcpListener,
) -> std::io::Result<()> {
    // Create a TCP listener

    let listener = std::net::TcpListener::bind(tcp_authorized.local_addr()?)?;
    let ip_address = listener.local_addr().unwrap();
    if user_authority.status == "Logged in" {
        println!("User is logged in");
    } else {
        println!("User is not logged in");
        return Ok(());
    }
    check_authority(&user_authority).await;
    // Start the Actix web server using HttpServer
    let listen = actix_web::HttpServer::new(|| {
        // Replaced HttpServer with actix_web::HttpServer
        actix_web::App::new() // Added actix_web:: prefix
            .service(index)
            .service(maintenance)
    })
    .listen(listener);
    listen?.run().await
}

// pub async fn tokio_ps_connection() -> Result<Client, Box<dyn Error>> {
//     let connection = "host=127.0.0.1 port=5432 user=postgres password=aakash123 dbname=postgres";
//     let (client, connection) = tokio_postgres::connect(connection, NoTls).await?;
//     rocket::tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });
//     Ok(client)
// }

async fn close_db(client_: &Client) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn add_maintainance_data(
    client: &Client,
    department: &str,
    tasks: Vec<String>,
    time: &str,
    date: &str,
    status: &str,
    description: &str,
    assigned_to: &str,
    assigned_by: &str,
    priority: &str,
) -> Result<(), Box<dyn Error>> {
    client
        .execute(
            "INSERT INTO $1 ( tasks, time, date, status, description, assigned_to, assigned_by, priority) VALUES ($2, $3, $4, $5, $6, $7, $8, $9) ",
            &[&department, &tasks, &time, &date, &status, &description, &assigned_to, &assigned_by, &priority],
        )
        .await?;
    Ok(())
}


pub async fn tokio_ps_connection() -> Result<Client, Box<dyn Error>> {
    let connection = "host=127.0.0.1 port=5432 user=postgres password=aakash123 dbname=postgres";
    let (client, connection) = tokio_postgres::connect(connection, NoTls).await?;
    rocket::tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn check_authority(user_authority: &Authority) {
    // Instantiate the Authority struct
    if user_authority.status == "Logged in" {
        println!("User is logged in");
    } else {
        println!("User is not logged in");
    }
}
