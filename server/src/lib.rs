use async_std::stream::StreamExt;
use chrono::Utc;
use futures_util::TryFutureExt;
use futures_util::{pin_mut, TryStreamExt};
use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::borrow::Borrow;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::{fs, thread};
use tokio::spawn;
use tokio_postgres::types::ToSql;
use tokio_postgres::Client; // Add this line to import the `spawn` function from the `rocket` crate
use tokio_postgres::NoTls; // Add this line to import the `spawn` function from the `rocket` crate // Add this import statement
pub const MAX_THREAD: u32 = 4;

enum Message {
    NewJob(Job),
    Terminate,
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub async fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker are terminating");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker{}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

async fn close_db(client_: &Client) -> Result<(), Box<dyn Error>> {
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

async fn authuntication(data: serde_json::Value) -> bool {
    let client = DB_connect().await.unwrap();
    let mut user_count = 0;
    let mut user_authority = Validity {
        username: "".to_string(),
        password: "".to_string(),
        token: "".to_string(),
        session_id: "".to_string(),
    };
    if user_count != 1 {
        return false;
    }

    if user_authority.session_id == "" && user_authority.token == "" {
        return true;
    }
    close_db(&client).await.unwrap();
    println!("Client: {:?}", client);
    true
}

pub async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let json_str = String::from_utf8_lossy(&buffer);
    let json: Value = serde_json::from_str(&json_str).unwrap_or(Value::Null);

    let (status_line, filename) = get_link(buffer.as_ref(), json).await;
    println!("Request: {}", status_line);
    println!("Request: {}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\n Content-length:{}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

struct Validity {
    username: String,
    password: String,
    session_id: String,
    token: String,
}

async fn get_link<'a>(buffer: &[u8], data: serde_json::Value) -> (&'a str, &'a str) {
    let post = b"POST / HTTP/1.1\r\n";
    let (mut status_line, mut filename) = ("HTTP/1.1 200 OK", "static_page/login.html");
    if buffer.starts_with(post) {
        let check = data_processing(buffer).await;

        match check {
            "login_ok" | "maintenance_ok" | "register_ok" => {
                return ("HTTP/1.1 200 OK", "static_page/index.html");
            }
            "forgot_password_ok" => return ("HTTP/1.1 200 OK", "static_page/login.html"),
            _ => {}
        }
    }
    let is_user_allowed: bool = authuntication(data).await;
    let buffer_str = std::str::from_utf8(buffer).unwrap();
    let start = buffer_str.find("GET ").unwrap_or(0) + 4; // 4 is the length of "GET "
    let end = buffer_str[start..].find(' ').unwrap_or(0) + start;
    let matching = (&buffer_str[start..end], is_user_allowed);

    match matching {
        ("/", _) => (status_line, filename),
        ("/admin", true) => {
            filename = "static_page/admin.html";
            (status_line, filename)
        }
        ("/dashboard", true) => {
            filename = "static_page/index.html";
            (status_line, filename)
        }
        ("/maintenance", true) => {
            filename = "static_page/Maintanence_page.html";
            (status_line, filename)
        }
        ("/profile", true) => {
            filename = "static_page/profile.html";
            (status_line, filename)
        }
        ("/settings", true) => {
            filename = "static_page/settings.html";
            (status_line, filename)
        }
        ("/forgot_password", false) => {
            filename = "static_page/forgot_password.html";
            (status_line, filename)
        }
        ("/register", _) => {
            filename = "static_page/register.html";
            (status_line, filename)
        }
        ("/login", _) => {
            filename = "static_page/login.html";
            (status_line, filename)
        }
        ("/logout", true) => {
            filename = "static_page/login.html";
            (status_line, filename)
        }
        ("/memo", true) => {
            filename = "static_page/memo.html";
            (status_line, filename)
        }
        (_, true) => ("HTTP/1.1 404 NOT FOUND", "static_page/404.html"),
        _ => ("HTTP/1.1 401 UNAUTHORIZED", "static_page/401.html"),
    }
}

async fn register_user(data: serde_json::Value) -> bool {
    let user = Validity {
        username: "admin".to_string(),
        password: "admin".to_string(),
        token: "123".to_string(),
        session_id: "123".to_string(),
    };
    true
}
async fn data_processing<'a>(buffer: &[u8]) -> &'a str {
    let buffer_str = std::str::from_utf8(buffer).unwrap();
    let start = buffer_str.find("POST ").unwrap_or(0) + 5; // 5 is the length of "POST "
    let end = buffer_str[start..].find(' ').unwrap_or(0) + start;
    let matching = &buffer_str[start..end];
    let data = &buffer_str[end..];
    let data = data.split("\r\n\r\n").collect::<Vec<&str>>()[1];
    let data = data.split("&").collect::<Vec<&str>>();
    let mut user = User {
        username: "".to_string(),
        password: "".to_string(),
    };
    for i in data {
        let i = i.split("=").collect::<Vec<&str>>();
        match i[0] {
            "username" => user.username = i[1].to_string(),
            "password" => user.password = i[1].to_string(),
            _ => {}
        }
    }
    #[derive(Serialize)]
    struct User {
        username: String,
        password: String,
    }

    let user = serde_json::to_value(user).unwrap();

    match matching {
        "/login" => {
            if login_user(user).await {
                return "login_ok";
            }
        }
        "/register" => {
            if register_user(user).await {
                return "register_ok";
            }
        }
        "/forgot_password" => {
            if change_user_info(user).await {
                return "forgot_password_ok";
            }
        }
        "/maintenance" => {
            if put_maintanence(user).await {
                return "maintenance_ok";
            }
        }
        _ => {}
    }
    ""
}
#[derive(serde::Deserialize)]
struct User {
    username: String,
    password: String,
}

async fn login_user(_data: serde_json::Value) -> bool {
    let user = Validity {
        username: "admin".to_string(),
        password: "admin".to_string(),
        token: "123".to_string(),
        session_id: "123".to_string(),
    };
    true
}

async fn get_user(data: serde_json::Value) -> serde_json::Value {
    let data = json!({
        "username": "admin",
        "password": "admin",
    });
    data
}

async fn change_user_info(data: serde_json::Value) -> bool {
    let user = get_user(data).await;
    let user = Validity {
        username: "admin".to_string(),
        password: "admin".to_string(),
        token: "123".to_string(),
        session_id: "123".to_string(),
    };
    true
}

async fn put_maintanence(data: serde_json::Value) -> bool {
    let user = Validity {
        username: "admin".to_string(),
        password: "admin".to_string(),
        token: "123".to_string(),
        session_id: "123".to_string(),
    };
    true
}

async fn DB_connect() -> Result<Client, Box<dyn Error>> {
    let connection = "host=127.0.0.1 port=5432 user=postgres password=aakash123 dbname=postgres";
    let (client, connection) = tokio_postgres::connect(connection, NoTls).await.unwrap();
    spawn(async move {
        // Replace `rocket::tokio::spawn` with `spawn`
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

fn Oauth2() {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.example.com/oauth2/token")
        .header("Authorization", "Basic Q WxhZGRpbjpvcGVuIHNlc2FtZQ==")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .unwrap_or_else(|error| panic!("Request failed: {}", error));
}
