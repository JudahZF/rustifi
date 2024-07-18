mod auth;

use reqwest;

#[tokio::main]
async fn main() {
    let result = reqwest::get("https://api.spotify.com/v1/search")
        .await
        .unwrap();
    println!("{:?}", result);
}
