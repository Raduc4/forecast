use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub location: Location,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "tz_id")]
    pub tz_id: String,
    #[serde(rename = "localtime_epoch")]
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(rename = "last_updated_epoch")]
    pub last_updated_epoch: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "temp_c")]
    pub temp_c: f64,
    #[serde(rename = "temp_f")]
    pub temp_f: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub condition: Condition,
    #[serde(rename = "wind_mph")]
    pub wind_mph: f64,
    #[serde(rename = "wind_kph")]
    pub wind_kph: f64,
    #[serde(rename = "wind_degree")]
    pub wind_degree: i64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pub pressure_mb: f64,
    #[serde(rename = "pressure_in")]
    pub pressure_in: f64,
    #[serde(rename = "precip_mm")]
    pub precip_mm: f64,
    #[serde(rename = "precip_in")]
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    #[serde(rename = "feelslike_c")]
    pub feelslike_c: f64,
    #[serde(rename = "feelslike_f")]
    pub feelslike_f: f64,
    #[serde(rename = "vis_km")]
    pub vis_km: f64,
    #[serde(rename = "vis_miles")]
    pub vis_miles: f64,
    pub uv: f64,
    #[serde(rename = "gust_mph")]
    pub gust_mph: f64,
    #[serde(rename = "gust_kph")]
    pub gust_kph: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("KEY").unwrap();
    let args: Vec<String> = env::args().collect();

    let url = format!(
        "http://api.weatherapi.com/v1/current.json?key={api_key}&q={}",
        args[1]
    );

    let forecast = reqwest::get(url).await?.json::<Root>().await?;

    println!("The templerature in {} {} is {} °C. Condition: {}", forecast.location.name, forecast.location.country, forecast.current.temp_c, forecast.current.condition.text);

    Ok(())
}
