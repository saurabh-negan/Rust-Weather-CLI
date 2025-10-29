use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

/// 🌦️ Simple Weather CLI using OpenWeatherMap API
#[derive(Parser, Debug)]
#[command(author, version, about = "Get current weather info for any city")]
struct Args {
    /// City name (e.g., London, Delhi, New York)
    #[arg(short, long)]
    city: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    weather: Vec<Weather>,
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    cod: serde_json::Value,  // ✅ this now works since serde_json is included
    message: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    let api_key = env::var("WEATHER_API_KEY")
        .expect("⚠️ Please set WEATHER_API_KEY in your .env file");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        args.city, api_key
    );

    let client = Client::new();
    let res = client.get(&url).send()?;

    if !res.status().is_success() {
        // If API returns an error JSON, parse it to show meaningful message
        let err: ApiError = res.json()?;
        eprintln!(
            "❌ Error fetching weather: {}",
            err.message.unwrap_or("Unknown error".to_string())
        );
        return Ok(());
    }

    let data: WeatherResponse = res.json()?;

    println!("🌍 Weather for: {}\n", data.name);
    println!("🌡️  Temperature: {:.1}°C", data.main.temp);
    println!("💧 Humidity: {}%", data.main.humidity);
    println!("☁️  Condition: {}", data.weather[0].description);

    Ok(())
}
