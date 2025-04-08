use std::fs::File;
use std::io::copy;
use tempfile::NamedTempFile;
use warp::Filter;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ImageRequest {
    image_url: String,
}

#[derive(Serialize)]
struct ImageResponse {
    result_image_url: String,
}

#[tokio::main]
async fn main() {
    // Endpoint POST /api/toanime
    let to_anime = warp::post()
        .and(warp::path("api"))
        .and(warp::path("toanime"))
        .and(warp::body::json())
        .and_then(handle_to_anime);

    // Start the server
    println!("Server running at http://localhost:3030");
    warp::serve(to_anime).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_to_anime(body: ImageRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let image_url = body.image_url;

    // Step 1: Download the image
    let client = Client::new();
    let response = match client.get(&image_url).send().await {
        Ok(res) => res,
        Err(_) => return Err(warp::reject::custom(ServerError("Failed to download image"))),
    };

    if !response.status().is_success() {
        return Err(warp::reject::custom(ServerError("Invalid image URL")));
    }

    // Save the image to a temporary file
    let mut temp_file = NamedTempFile::new().unwrap();
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(_) => return Err(warp::reject::custom(ServerError("Failed to read image bytes"))),
    };
    copy(&mut bytes.as_ref(), &mut temp_file).unwrap();

    // Step 2: Convert image using Puppeteer-like logic (placeholder)
    let result_image_url = convert_image_to_anime(temp_file.path().to_str().unwrap());

    // Return the result
    Ok(warp::reply::json(&ImageResponse {
        result_image_url,
    }))
}

fn convert_image_to_anime(image_path: &str) -> String {
    // Placeholder logic for image conversion
    // In a real-world scenario, you would integrate with a service or library here.
    format!("https://example.com/processed/{}", image_path.split('/').last().unwrap_or("output.jpg"))
}

// Custom error handling
#[derive(Debug)]
struct ServerError(&'static str);

impl warp::reject::Reject for ServerError {}

#[tokio::main]
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(e) = err.find::<ServerError>() {
        Ok(warp::reply::with_status(
            e.0.to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Unhandled error".to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
