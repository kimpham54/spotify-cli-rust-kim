// use json::*;
// use reqwest::header::{ORIGIN, REFERER, USER_AGENT};
// use reqwest::{self, Client, status};
// use std::io::Read;
// use std::net::TcpListener;
// use std::sync::Mutex;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

// Headers
const CONTENTTYPE: &str = "application/json";
const OAUTH_TOKEN: &str = "Bearer BQCR0DEVZwa3Hsrr4faNLHNBO7TL_tGZrpcMdyBRbxZnB4Uks01HBiDlQDEJozOC_wbX8hiGnrG074lBFcGOycL6NZx_9xbvnKEQHoeXcN_SdwWhS1EACMpfLnuRhHogxtnZdQwOyInoLZGvrJ_qm1hLaPL5BiUUQl8pGOspao-0xhGq";

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[tokio::main]
async fn main() {
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&market=US&type=track&limit=5",
        query = "door maverick howe gelb"
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, OAUTH_TOKEN)
        .header(CONTENT_TYPE, CONTENTTYPE)
        .header(ACCEPT, CONTENTTYPE)
        .send()
        .await
        .unwrap();
    // .text()
    // .await;
    println!("{:#?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}
