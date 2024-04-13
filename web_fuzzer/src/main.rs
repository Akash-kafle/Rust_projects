use std::io;
use std::error::Error;
use reqwest;

const dictonary: [&str; 3] = ["http://www.google.com", "http://www.facebook.com", "http://www.twitter.com"];

fn main() {
   main_connection();
}


fn Connect_to_url(url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get(url).send()?;
    println!("Status: {}", res.status());
    Ok(())
}

fn input_url() -> String {
    let mut url = String::new();
    println!("Enter the URL you want to connect to: ");
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url
}

fn main_connection()-> i32{
    let url = input_url();
    match Connect_to_url(&url) {
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