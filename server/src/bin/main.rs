use main_::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize}; // Added import for Serialize
use std::error::Error; // Added import for Error
use std::fs; // Added import for fs
use tokio_postgres::Client; // Add the missing import statement for TcpListener
use tokio_postgres::NoTls; // Add the missing import statement for NoTls

#[derive(Serialize)]
struct LoginResponse {
    status: &'static str,
    message: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[get("/")]
async fn login() -> impl Responder {
    let content = fs::read_to_string("static_page/login.html")
        .unwrap_or_else(|_| String::from("Error reading login.html"));
    HttpResponse::Ok().body(content)
}

#[actix_web::post("/login")]
async fn login_(req: web::Json<RegisterRequest>) -> impl Responder {
    let (username, password) = (req.username.clone(), req.password.clone());

    // Get a connection from the connection pool
    let connection = tokio_ps_connection()
        .await
        .expect("Failed to obtain database connection"); // Add the `await` keyword to await the future and retrieve the result

    // Hash the password (implement this using a password hashing library)
    //let hashed_password = hash_password(&password);

    let query = "SELECT * FROM maintenance WHERE username = $1 AND password = $2";
    let rows = connection.query(query, &[&username, &password]).await;
    let total_users: i32 = rows.as_ref().unwrap().len() as i32;
    let admin = rows.as_ref().unwrap().get(3);

    match (rows, total_users) {
        (Ok(_), 1) => {
            main_::Authority {
                username: username,
                password: password,
                status: "Logged in".to_string(),
                login_time: (Utc::now()).to_string(),
                logout_time: "".to_string(),
                Admin: false,
            };
            HttpResponse::Ok().json(LoginResponse {
                status: "success",
                message: "Login successfully".to_string(),
            })
        }
        (Err(e), _) => {
            println!("Error querying the database: {}", e.to_string());
            HttpResponse::InternalServerError().json(LoginResponse {
                status: "error",
                message: format!("Login failed: {}", e.to_string()),
            })
        }
        (_, _) => HttpResponse::Unauthorized().json(LoginResponse {
            status: "error",
            message: "Invalid username or password".to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let user_authority = main_::Authority {
        username: "".to_string(),
        password: "".to_string(),
        status: "Not logged in".to_string(),
        login_time: "".to_string(),
        logout_time: "".to_string(),
        Admin: false,
    };
    let listener = std::net::TcpListener::bind("127.0.0.1:8087")?; // Replaced TcpListener with actix_web::TcpListener and added await
    let cloned_listener = listener.try_clone()?; // Clone the TcpListener
    let listen = actix_web::HttpServer::new(|| {
        // Replaced HttpServer with actix_web::HttpServer
        actix_web::App::new() // Added actix_web:: prefix
            .service(login)
            .service(login_)
            .service(register)
            .service(register_)
            .service(maintenance)
            .service(index)
            
    })
    .listen(cloned_listener); // Use the cloned TcpListener

    Ok(())
}

async fn create_table_user(client: &Client) -> Result<(), Box<dyn Error>> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS maintenance (
                id SERIAL PRIMARY KEY,
                username VARCHAR NOT NULL,
                password VARCHAR NOT NULL
            )",
            &[],
        )
        .await?;
    Ok(())
}



async fn close_db(client_: &Client) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[get("/register_page")]
async fn register() -> impl Responder {
    let content = fs::read_to_string("static_page/register.html")
        .unwrap_or_else(|_| String::from("Error reading register.html"));
    HttpResponse::Ok().body(content)
}

#[actix_web::post("/register")]
async fn register_(req: web::Json<RegisterRequest>) -> HttpResponse {
    let (username, password) = (req.username.clone(), req.password.clone());

    let _connection = tokio_ps_connection().await.unwrap(); // Connection to the database
    let table = create_table_user(&_connection).await; // Create the table
    let insert = insert_data_user(&_connection, &username, &password).await; // Insert the data
    let close_db = close_db(&_connection).await; // Close the database connection

    if close_db.is_err() {
        println!("Error closing the database connection");
    }

    if let Err(e) = insert {
        HttpResponse::InternalServerError().body(format!("Error inserting data: {}", e));
    }
    match table {
        Ok(_) => {
            main_::Authority {
                username: username,
                password: password,
                status: "Registered".to_string(),
                login_time: (Utc::now()).to_string(),
                logout_time: "".to_string(),
                Admin: false,
            };

            HttpResponse::Ok().body("Table created successfully")
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating table: {}", e)),
    }
}


async fn insert_data_user(
    client: &Client,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    client
        .execute(
            "INSERT INTO maintenance (username, password) VALUES ($1, $2)",
            &[&username, &password],
        )
        .await?;
    Ok(())
}

#[get("/dashboard/maintenance-data")]
async fn get_maintenance_data() -> impl Responder {
    let user_authority = Authority {
        username: "".to_string(),
        password: "".to_string(),
        status: "Not logged in".to_string(),
        login_time: "".to_string(),
        logout_time: "".to_string(),
        Admin: false,
    };

    let _client = tokio_ps_connection().await.unwrap(); // Added await keyword
    let dummy_data = vec![
        MaintenanceData {
            department: "IT".to_string(),
            tasks: vec!["Task 1".to_string(), "Task 2".to_string()],
            time: "10:00".to_string(),
            date: "2021-01-01".to_string(),
            status: "Pending".to_string(),
            description: "This is a description".to_string(),
            assigned_to: "Aakash".to_string(),
            assigned_by: "Aakash".to_string(),
            priority: "High".to_string(),
        },
        MaintenanceData {
            department: "HR".to_string(),
            tasks: vec!["Task 1".to_string(), "Task 2".to_string()],
            time: "10:00".to_string(),
            date: "2021-01-01".to_string(),
            status: "Pending".to_string(),
            description: "This is a description".to_string(),
            assigned_to: "Aakash".to_string(),
            assigned_by: "Aakash".to_string(),
            priority: "High".to_string(),
        },
    ];

    web::Json(dummy_data)
}

#[get("/maintenance")]
async fn maintenance() -> HttpResponse {
    let user_authority = Authority {
        username: "".to_string(),
        password: "".to_string(),
        status: "Not logged in".to_string(),
        login_time: "".to_string(),
        logout_time: "".to_string(),
        Admin: false,
    };
    check_authority(&user_authority).await;
    // Load the content from the "maintenance.html" file
    let content = fs::read_to_string("static_page/Maintanence_page.html")
        .unwrap_or_else(|_| String::from("Error reading maintenance.html"));

    HttpResponse::Ok().body(content)
}

#[get("/dashboard")]
async fn index() -> impl Responder {
    let user_authority = Authority {
        username: "".to_string(),
        password: "".to_string(),
        status: "Not logged in".to_string(),
        login_time: "".to_string(),
        logout_time: "".to_string(),
        Admin: false,
    };
    check_authority(&user_authority).await;
    // Load the content from the "index.html" file
    let content = fs::read_to_string("static_page/index.html")
        .unwrap_or_else(|_| String::from("Error reading index.html"));

    HttpResponse::Ok().body(content)
}