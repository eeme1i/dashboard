use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use tokio::sync::RwLock;

const LOCATION_CACHE_FILE: &str = "cache/location_cache.json";

#[derive(Deserialize, Serialize, Clone)]
pub struct Coordinates {
    lat: f64,
    lon: f64,
}

impl Coordinates {
    pub fn new(lat: f64, lon: f64) -> Self {
        Coordinates { lat, lon }
    }
    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

#[derive(Deserialize, Serialize)]
struct GeocodeResult {
    lat: String,
    lon: String,
}

#[derive(Deserialize, Serialize)]
struct GeocodeResponse(Vec<GeocodeResult>);

type LocationCache = HashMap<String, Coordinates>;

static CACHE: Lazy<RwLock<LocationCache>> = Lazy::new(|| RwLock::new(LocationCache::new()));

async fn load_cache() -> Result<LocationCache, Box<dyn std::error::Error + Send + Sync>> {
    match tokio::fs::read_to_string(LOCATION_CACHE_FILE).await {
        Ok(data) => {
            let cache: LocationCache = serde_json::from_str(&data)?;
            Ok(cache)
        }
        Err(_) => Ok(LocationCache::new()),
    }
}

async fn save_cache(cache: &LocationCache) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = serde_json::to_string(cache)?;
    match tokio::fs::create_dir_all("cache").await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create cache directory: {}", e);
            return Err(Box::new(e));
        }
    }
    tokio::fs::write(LOCATION_CACHE_FILE, data).await?;
    Ok(())
}

pub async fn get_coordinates(
    location: &str,
) -> Result<Coordinates, Box<dyn std::error::Error + Send + Sync>> {
    {
        let cache_read = CACHE.read().await;
        if let Some(coords) = cache_read.get(location) {
            return Ok(coords.clone());
        }
    }
    let mut cache_write = CACHE.write().await;

    if cache_write.is_empty() {
        match load_cache().await {
            Ok(file_cache) => {
                *cache_write = file_cache;
            }
            Err(e) => {
                eprintln!("Failed to load location cache: {}", e);
            }
        }
    }

    if let Some(coords) = cache_write.get(location) {
        return Ok(coords.clone());
    }

    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        location
    );
    let client = reqwest::Client::new();

    let response = match client
        .get(&url)
        .header("User-Agent", "wreport (eemeliruoh@gmail.com)") // <-- Required!
        .send()
        .await
    {
        Ok(resp) => match resp.json::<GeocodeResponse>().await {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to parse geocode JSON response: {:#}", e);
                return Err(Box::new(e));
            }
        },
        Err(e) => {
            // Print the full error chain to stderr so docker logs capture the root cause
            eprintln!("Failed to send geocode request for url ({}): {}", url, e);
            let mut source = e.source();
            while let Some(s) = source {
                eprintln!("  caused by: {}", s);
                source = s.source();
            }
            return Err(Box::new(e));
        }
    };

    if let Some(result) = response.0.first() {
        let lat: f64 = result.lat.parse()?;
        let lon: f64 = result.lon.parse()?;
        let coordinates = Coordinates::new(lat, lon);
        cache_write.insert(location.to_string(), coordinates.clone());
        if let Err(e) = save_cache(&*cache_write).await {
            eprintln!("Failed to save location cache: {}", e);
        }
        Ok(coordinates)
    } else {
        Err("No results found".into())
    }
}
