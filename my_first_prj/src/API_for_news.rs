use reqwest::blocking::get;

fn main() {
    let url = "https://api.example.com/news";

    match get(url) {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().unwrap();
                println!("API response: {}", body);
            } else {
                println!("API request failed with status code: {}", response.status());
            }
        }
        Err(error) => {
            println!("API request failed: {}", error);
        }
    }
}
