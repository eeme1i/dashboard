use chrono_tz::Tz;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::location::Coordinates;

const WEATHER_CACHE_FILE: &str = "cache/weather_cache.json";
const WEATHER_SUMMARY_CACHE_FILE: &str = "cache/weather_summary_cache.json";

static WEATHER_CACHE: Lazy<RwLock<WeatherCache>> = Lazy::new(|| {
    RwLock::new(WeatherCache {
        cache: std::collections::HashMap::new(),
    })
});
static WEATHER_SUMMARY_CACHE: Lazy<RwLock<WeatherSummaryCache>> = Lazy::new(|| {
    RwLock::new(WeatherSummaryCache {
        cache: std::collections::HashMap::new(),
    })
});

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Geometry {
    pub coordinates: [f64; 3],
    #[serde(rename = "type")]
    pub geometry_type: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastTimeInstant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_pressure_at_sea_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_high: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_medium: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew_point_temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_area_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_from_direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_of_gust: Option<f64>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastTimePeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_precipitation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_thunder: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultraviolet_index_clear_sky_max: Option<f64>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastSummary {
    pub symbol_code: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastDetails {
    pub summary: ForecastSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimePeriod>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct InstantDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ForecastTimeInstant>,
}
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TimeSeriesData {
    pub instant: InstantDetails,
    #[serde(rename = "next_1_hours")]
    pub next_1_hours: Option<ForecastDetails>,
    #[serde(rename = "next_6_hours")]
    pub next_6_hours: Option<ForecastDetails>,
    #[serde(rename = "next_12_hours")]
    pub next_12_hours: Option<ForecastDetails>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TimeSeriesEntry {
    pub time: String,
    pub data: TimeSeriesData,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastUnits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_pressure_at_sea_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_temperature_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_high: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_low: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_area_fraction_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dew_point_temperature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog_area_fraction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_amount_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_precipitation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability_of_thunder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultraviolet_index_clear_sky_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_from_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_of_gust: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ForecastMeta {
    pub units: ForecastUnits,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Properties {
    pub meta: ForecastMeta,
    pub timeseries: Vec<TimeSeriesEntry>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct WeatherResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherCacheItem {
    pub time: chrono::DateTime<chrono::Utc>,
    pub weather: WeatherResponse,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherCache {
    pub cache: std::collections::HashMap<String, WeatherCacheItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherSummaryCacheItem {
    pub time: chrono::DateTime<chrono::Utc>,
    pub summary: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WeatherSummaryCache {
    pub cache: std::collections::HashMap<String, WeatherSummaryCacheItem>,
}

async fn load_weather_cache() -> Result<WeatherCache, Box<dyn std::error::Error + Send + Sync>> {
    match tokio::fs::read_to_string(WEATHER_CACHE_FILE).await {
        Ok(data) => {
            let cache: WeatherCache = serde_json::from_str(&data)?;
            Ok(cache)
        }
        Err(_) => Ok(WeatherCache {
            cache: std::collections::HashMap::new(),
        }),
    }
}

async fn clear_useless_weather_cache(
    cache: &mut WeatherCache,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let current_time = chrono::Utc::now();
    cache
        .cache
        .retain(|_, entry| current_time.signed_duration_since(entry.time).num_seconds() < 600);
    Ok(())
}

async fn save_weather_cache(
    cache: &WeatherCache,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = serde_json::to_string(cache)?;
    match tokio::fs::create_dir_all("cache").await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create cache directory: {}", e);
            return Err(Box::new(e));
        }
    }
    tokio::fs::write(WEATHER_CACHE_FILE, data).await?;
    Ok(())
}

async fn load_weather_summary_cache()
-> Result<WeatherSummaryCache, Box<dyn std::error::Error + Send + Sync>> {
    match tokio::fs::read_to_string(WEATHER_SUMMARY_CACHE_FILE).await {
        Ok(data) => {
            let cache: WeatherSummaryCache = serde_json::from_str(&data)?;
            Ok(cache)
        }
        Err(_) => Ok(WeatherSummaryCache {
            cache: std::collections::HashMap::new(),
        }),
    }
}

async fn save_weather_summary_cache(
    cache: &WeatherSummaryCache,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = serde_json::to_string(cache)?;
    match tokio::fs::create_dir_all("cache").await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to create cache directory: {}", e);
            return Err(Box::new(e));
        }
    }
    tokio::fs::write(WEATHER_SUMMARY_CACHE_FILE, data).await?;
    Ok(())
}

fn coord_key(coord: &Coordinates) -> String {
    format!("{:.4},{:.4}", coord.lat(), coord.lon())
}

#[derive(Serialize)]
pub struct PublicWeatherResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub geometry: Geometry,
    pub properties: PublicProperties,
}

#[derive(Serialize)]
pub struct PublicProperties {
    pub meta: ForecastMeta,
    pub timeseries: Vec<TimeSeriesEntry>,
}

pub async fn fetch_weather(
    coord: &Coordinates,
) -> Result<PublicWeatherResponse, Box<dyn std::error::Error + Send + Sync>> {
    {
        let cache_read = WEATHER_CACHE.read().await;
        if let Some(entry) = cache_read.cache.get(&coord_key(coord)) {
            let current_time = chrono::Utc::now();
            if current_time.signed_duration_since(entry.time).num_seconds() < 600 {
                return Ok(PublicWeatherResponse {
                    response_type: entry.weather.response_type.clone(),
                    geometry: entry.weather.geometry.clone(),
                    properties: PublicProperties {
                        meta: entry.weather.properties.meta.clone(),
                        timeseries: entry.weather.properties.timeseries.clone(),
                    },
                });
            }
        }
    }
    let mut cache_write = WEATHER_CACHE.write().await;

    if cache_write.cache.is_empty() {
        match load_weather_cache().await {
            Ok(file_cache) => {
                *cache_write = file_cache;
            }
            Err(e) => {
                eprintln!("Failed to load weather cache: {}", e);
            }
        }
    }

    let key = coord_key(coord);

    if let Some(entry) = cache_write.cache.get(&key) {
        let current_time = chrono::Utc::now();
        if current_time.signed_duration_since(entry.time).num_seconds() < 600 {
            return Ok(PublicWeatherResponse {
                response_type: entry.weather.response_type.clone(),
                geometry: entry.weather.geometry.clone(),
                properties: PublicProperties {
                    meta: entry.weather.properties.meta.clone(),
                    timeseries: entry.weather.properties.timeseries.clone(),
                },
            });
        }
    }
    let url = format!(
        "https://api.met.no/weatherapi/locationforecast/2.0/complete?lat={}&lon={}",
        coord.lat(),
        coord.lon()
    );

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "weather for home (eemeliruoh@gmail.com)")
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;

    let new_entry = WeatherCacheItem {
        time: chrono::Utc::now(),
        weather: response.clone(),
    };

    cache_write.cache.insert(key, new_entry);
    clear_useless_weather_cache(&mut cache_write).await?;
    save_weather_cache(&cache_write).await?;
    Ok(PublicWeatherResponse {
        response_type: response.response_type,
        geometry: response.geometry,
        properties: PublicProperties {
            meta: response.properties.meta,
            timeseries: response.properties.timeseries,
        },
    })
}

#[derive(Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

pub async fn summarize_weather(
    weather: &PublicWeatherResponse,
    timezone: Option<String>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // Create a cache key from the weather coordinates
    let key = format!(
        "{:.4},{:.4}",
        weather.geometry.coordinates[1], weather.geometry.coordinates[0]
    );

    {
        let cache_read = WEATHER_SUMMARY_CACHE.read().await;
        if let Some(entry) = cache_read.cache.get(&key) {
            let current_time = chrono::Utc::now();
            // Cache summaries for 10 minutes (600 seconds)
            if current_time.signed_duration_since(entry.time).num_seconds() < 600 {
                return Ok(entry.summary.clone());
            }
        }
    }

    // Try to load from cache
    let mut summary_cache = WEATHER_SUMMARY_CACHE.write().await;

    if summary_cache.cache.is_empty() {
        match load_weather_summary_cache().await {
            Ok(file_cache) => {
                *summary_cache = file_cache;
            }
            Err(e) => {
                eprintln!("Failed to load weather summary cache: {}", e);
            }
        }
    }

    if let Some(entry) = summary_cache.cache.get(&key) {
        let current_time = chrono::Utc::now();
        // Cache summaries for 10 minutes (600 seconds)
        if current_time.signed_duration_since(entry.time).num_seconds() < 600 {
            return Ok(entry.summary.clone());
        }
    }

    // read from dotenv
    let google_aistudio_api_key = dotenv::var("GOOGLE_AISTUDIO_API_KEY")?;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemma-3-27b-it:generateContent?key={}",
        google_aistudio_api_key
    );

    let client = reqwest::Client::new();

    let prompt = build_prompt(weather, timezone).await?;

    let request_body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": prompt,
                    }
                ]
            }
        ],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 1000,
            "topP": 0.95,
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("API request failed: {}", error_text).into());
    }

    let parsed_response: GenerateContentResponse = response.json().await?;

    let summary = parsed_response
        .candidates
        .first()
        .ok_or("No candidates in response")?
        .content
        .parts
        .first()
        .ok_or("No parts in content")?
        .text
        .clone();

    // Save the summary to cache
    let new_entry = WeatherSummaryCacheItem {
        time: chrono::Utc::now(),
        summary: summary.clone(),
    };

    summary_cache.cache.insert(key, new_entry);
    save_weather_summary_cache(&summary_cache).await?;

    Ok(summary)
}

async fn build_prompt(
    weather: &PublicWeatherResponse,
    timezone: Option<String>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let now = if let Some(timezone) = timezone {
        let tz: Tz = timezone.parse()?;
        chrono::Utc::now().with_timezone(&tz)
    } else {
        chrono::Utc::now().with_timezone(&chrono_tz::UTC)
    };

    let time_str = now.format("%H:%M").to_string();

    let current = &weather
        .properties
        .timeseries
        .get(0)
        .ok_or("No timeseries data available")?
        .data;

    let instant = current
        .instant
        .details
        .as_ref()
        .ok_or("No instant data available")?;

    let next_1h = current
        .next_1_hours
        .as_ref()
        .and_then(|f| f.details.as_ref());
    let next_6h = current
        .next_6_hours
        .as_ref()
        .and_then(|f| f.details.as_ref());
    let next_12h = current
        .next_12_hours
        .as_ref()
        .and_then(|f| f.details.as_ref());

    // Helper to format optional f64 values as "N/A" when missing
    let fmt = |opt: Option<f64>| -> String {
        opt.map(|v| format!("{:.1}", v))
            .unwrap_or_else(|| "N/A".to_string())
    };

    let temp_str = fmt(instant.air_temperature);
    let wind_str = fmt(instant.wind_speed);
    let gust_str = fmt(instant.wind_speed_of_gust);
    let humidity_str = fmt(instant.relative_humidity);
    let cloud_str = fmt(instant.cloud_area_fraction);
    let fog_str = fmt(instant.fog_area_fraction);

    let next1_summary = current
        .next_1_hours
        .as_ref()
        .map(|f| f.summary.symbol_code.clone())
        .unwrap_or_else(|| "N/A".to_string());
    let next1_precip = fmt(next_1h.and_then(|d| d.precipitation_amount));
    let next1_prob = fmt(next_1h.and_then(|d| d.probability_of_precipitation));

    let next6_summary = current
        .next_6_hours
        .as_ref()
        .map(|f| f.summary.symbol_code.clone())
        .unwrap_or_else(|| "N/A".to_string());
    let next6_max = fmt(next_6h.and_then(|d| d.air_temperature_max));
    let next6_min = fmt(next_6h.and_then(|d| d.air_temperature_min));
    let next6_precip = fmt(next_6h.and_then(|d| d.precipitation_amount));
    let next6_prob = fmt(next_6h.and_then(|d| d.probability_of_precipitation));

    let next12_summary = current
        .next_12_hours
        .as_ref()
        .map(|f| f.summary.symbol_code.clone())
        .unwrap_or_else(|| "N/A".to_string());
    let next12_max = fmt(next_12h.and_then(|d| d.air_temperature_max));
    let next12_min = fmt(next_12h.and_then(|d| d.air_temperature_min));
    let next12_precip = fmt(next_12h.and_then(|d| d.precipitation_amount));
    let next12_prob = fmt(next_12h.and_then(|d| d.probability_of_precipitation));

    let prompt = format!(
        "Generate a concise, natural weather description for a dashboard. Keep it under 25 words.\n\nCurrent time: {}\n\nCurrent conditions:\nTemperature: {}°C\nWind: {} m/s with gusts of {} m/s\nHumidity: {}%\nCloud area fraction: {}%\nFog area fraction: {}%\n\nForecast 1 hour:\nSummary: {}\nPrecipitation: {} mm with a probability of {}%\n\nForecast 6 hours:\nSummary: {}\nMax Temperature: {}°C\nMin Temperature: {}°C\nPrecipitation: {} mm with a probability of {}%\n\nForecast 12 hours:\nSummary: {}\nMax Temperature: {}°C\nMin Temperature: {}°C\nPrecipitation: {} mm with a probability of {}%\n\nRequirements:\n- Be conversational and friendly.\n- Do not mention the current temperature. It will be displayed seperately.\n- Upcoming temperatures should be included if there is a significant change.\n- For wind: Use descriptive terms (calm, light, moderate, strong, extreme) - NEVER use specific values.\n- Use natural language, no technical jargon.\n- NO EMOJIS.\n\nGenerate description:",
        time_str,
        temp_str,
        wind_str,
        gust_str,
        humidity_str,
        cloud_str,
        fog_str,
        next1_summary,
        next1_precip,
        next1_prob,
        next6_summary,
        next6_max,
        next6_min,
        next6_precip,
        next6_prob,
        next12_summary,
        next12_max,
        next12_min,
        next12_precip,
        next12_prob,
    );

    Ok(prompt)
}
