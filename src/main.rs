use std::env;

use base64::encode;
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SpotifyAPIToken {
    access_token: String,
    token_type: String,
    expires_in: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    let authorization = encode(format!("{}:{}", client_id, client_secret));

    let access_token = Client::new()
        .post("https://accounts.spotify.com/api/token")
        .form(&[("grant_type", "client_credentials")])
        .header("Authorization", format!("Basic {}", authorization))
        .send()
        .await?
        .json::<SpotifyAPIToken>()
        .await?;

    println!("{:#?}", access_token);
    Ok(())
}
