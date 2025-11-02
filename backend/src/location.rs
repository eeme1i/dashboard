use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    let mut cache = load_cache().await?;

    if let Some(coords) = cache.get(location) {
        return Ok(coords.clone());
    }
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        location
    );
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "wreport (eemeliruoh@gmail.com)") // <-- Required!
        .send()
        .await?
        .json::<GeocodeResponse>()
        .await?;

    if let Some(result) = response.0.first() {
        let lat: f64 = result.lat.parse()?;
        let lon: f64 = result.lon.parse()?;
        let coordinates = Coordinates::new(lat, lon);
        cache.insert(location.to_string(), coordinates.clone());
        save_cache(&cache).await?;
        Ok(coordinates)
    } else {
        Err("No results found".into())
    }
}
