use std::io;
use std::error::Error;
use reqwest;

const dictonary: [&str; 3] = ["http://www.google.com", "http://www.facebook.com", "http://www.twitter.com"];

fn main() {
   main_connection();
}

async fn Check_end_points() {
            for i in 0..dictonary.len() {
                match Connect_to_url(dictonary[i]).await {
                    Ok(_) => {
                        println!("Connection successful");
                        let endpoints = Get_endpoints(dictonary[i]).await;
                        println!("Available endpoints for {}: {:?}", dictonary[i], endpoints);
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }

async fn Get_endpoints(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
            let client = reqwest::Client::new();
            let res = client.get(url).send().await?;
            let body = res.text().await?;
            let endpoints = parse_endpoints(&body);
            Ok(endpoints)
        }

fn parse_endpoints(body: &str) -> Vec<String> {
            // Implement your logic to parse the endpoints from the HTML body
            // and return them as a vector of strings
            // Example:
            // let mut endpoints = Vec::new();
            // endpoints.push("/home");
            // endpoints.push("/about");
            // endpoints.push("/contact");
            // endpoints
            unimplemented!()
        }

async fn Connect_to_url(url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await?;
    println!("Status: {}", res.status());
    Ok(())
}

fn input_url() -> String {
    let mut url = String::new();
    println!("Enter the URL you want to connect to: ");
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url
}

async fn main_connection()-> i32{
    let url = input_url();
    match Connect_to_url(&url).await {
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