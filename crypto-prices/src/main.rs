/*
 * A simple program to fetch the latest crypto listings
 * Run with `export CMC_API_KEY="<my_api_key>" && cargo run`
 */
use reqwest::{header, Error};
use std::env;

async fn get_request() -> Result<(), Error> {
    // Set Coinmarketcap API endpoint url
    let url = "https://sandbox-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

    // Get the API key from environment variable
    let api_key = env!("CMC_API_KEY");

    // Set up the headers for the request
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-CMC_PRO_API_KEY",
        header::HeaderValue::from_static(api_key),
    );

    // make the request to fetch the latest prices
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?;

    // get the response status code
    let status = response.status();

    // get the JSON from the response body
    let resp = response.json::<serde_json::Value>().await?;

    println!("Response: {:#?}", resp);
    println!("URL: {:#?}", url);
    println!("API key: {:#?}", api_key);
    println!("Status: {:#?}", status);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    get_request().await?;
    Ok(())
}
