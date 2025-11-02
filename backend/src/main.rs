use axum::{
    Json, Router,
    extract::{Path, Query},
    http::Method,
    routing::get,
};
use reqwest::StatusCode;
use tower_http::cors::{Any, CorsLayer};

use crate::{location::Coordinates, weather::PublicWeatherResponse};

mod location;
mod weather;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/location/{location}", get(handle_location))
        .route("/api/weather/{location}", get(handle_weather))
        .route(
            "/api/weather/{location}/summary",
            get(handle_summarize_weather),
        )
        .route(
            "/api/weather/{location}/current_temperature",
            get(handle_current_temperature),
        )
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2001").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn handle_location(
    Path(location): Path<String>,
) -> Result<Json<Coordinates>, (StatusCode, String)> {
    match location::get_coordinates(&location).await {
        Ok(coords) => Ok(Json(coords)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[axum::debug_handler]
async fn handle_weather(
    Path(location): Path<String>,
) -> Result<Json<PublicWeatherResponse>, (StatusCode, String)> {
    match location::get_coordinates(&location).await {
        Ok(coords) => match weather::fetch_weather(&coords).await {
            Ok(weather_data) => Ok(Json(weather_data)),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(serde::Serialize)]
struct WeatherSummary {
    summary: String,
}

#[derive(serde::Deserialize)]
struct WeatherQuery {
    #[serde(default)]
    timezone: Option<String>,
}

#[axum::debug_handler]
async fn handle_summarize_weather(
    Path(location): Path<String>,
    Query(params): Query<WeatherQuery>,
) -> Result<Json<WeatherSummary>, (StatusCode, String)> {
    match location::get_coordinates(&location).await {
        Ok(coords) => match weather::fetch_weather(&coords).await {
            Ok(weather_data) => {
                match weather::summarize_weather(&weather_data, params.timezone).await {
                    Ok(summary) => Ok(Json(WeatherSummary { summary })),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(serde::Serialize)]
struct CurrentTemperature {
    temperature_celsius: f64,
}

async fn handle_current_temperature(
    Path(location): Path<String>,
) -> Result<Json<CurrentTemperature>, (StatusCode, String)> {
    match location::get_coordinates(&location).await {
        Ok(coords) => match weather::fetch_weather(&coords).await {
            Ok(weather_data) => {
                let current_temp = weather_data
                    .properties
                    .timeseries
                    .first()
                    .and_then(|ts| ts.data.instant.details.as_ref())
                    .and_then(|details| details.air_temperature);

                match current_temp {
                    Some(temp) => Ok(Json(CurrentTemperature {
                        temperature_celsius: temp,
                    })),
                    None => Err((
                        StatusCode::NOT_FOUND,
                        "Temperature data not found".to_string(),
                    )),
                }
            }
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
