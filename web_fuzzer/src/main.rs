use std::io;
use std::error::Error;
use reqwest;

const dictonary: [&str; 3] = ["http://www.google.com", "http://www.facebook.com", "http://www.twitter.com"];

fn main() {
   main_connection();
}

async fn check_end_points() {
            for i in 0..dictonary.len() {
                match get_endpoints(dictonary[i]).await {
                    Ok(_) => {
                        println!("Connection successful");
                        let endpoints = get_endpoints(dictonary[i]).await;
                        println!("Available endpoints for {}: {:?}", dictonary[i], endpoints);
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }

async fn get_endpoints(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
            let client = reqwest::Client::new();
            let res = client.get(url).send().await?;
            let body = res.text().await?;
            let endpoints = parse_endpoints(&body);
            Ok(endpoints)
        }

fn parse_endpoints(body: &str) -> Vec<String> {
    let mut endpoints = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    for line in lines {
        if line.starts_with("Endpoint:") {
            let endpoint = line.trim_start_matches("Endpoint:").trim();
            endpoints.push(endpoint.to_string());

        }
    }
    endpoints
}

fn input_url() -> String {
    let mut url = String::new();
    println!("Enter the URL you want to connect to: ");
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url
}

async fn main_connection()-> i32{
    let url = input_url();
    match get_endpoints(&url).await {
        Ok(_) => {
            println!("Connection successful");
            return 1;
        },
        Err(e) => {
            println!("Error: {}", e);
            return 0;
        }
    }
}