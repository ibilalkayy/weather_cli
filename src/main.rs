use reqwest::blocking::Client;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Weather {
    #[clap(subcommand)]
    check: Check,
}

#[derive(Debug, Subcommand)]
enum Check {
    /// Show the current weather of a city
    Current(City),

    /// Display severe weather alerts for a city.
    Alerts(City),

    /// Compare weather between multiple cities.
    Compare(Cities),
}

#[derive(Debug, Parser)]
struct City {
    /// Specify name of the city
    #[clap(short, long)]
    city: String,
}

#[derive(Debug, Parser)]
struct Cities {
    /// Specify the name of the cities
    #[clap(short, long)]
    cities: String,
}

fn main() {
    let weather = Weather::parse();
    match weather.check {
        Check::Current(details) => {
            let http_client = Client::new();
            let url = format!("http://api.weatherstack.com/current?access_key=api_key&query={}", details.city);
            let http_result = http_client.get(&url).send();

            match http_result {
                Ok(response) => {
                    let text_result = response.text();
                    match text_result {
                        Ok(text) => {
                            match serde_json::from_str::<Value>(&text) {
                                Ok(json) => {
                                    if let Some(location) = json.get("location") {
                                        if let Some(current) = json.get("current") {
                                            println!("Weather in {}", location["name"].as_str().unwrap());
                                            println!("Temperature: {}C", current["temperature"]);
                                            println!("Humidity: {}%", current["humidity"]);
                                            println!("Wind Speed: {} km/h", current["wind_speed"]);
                                        }
                                    }
                                },
                                Err(err) => println!("Failed to get the JSON data: {}", err)
                            }
                        },
                        Err(err) => println!("Failed to get the text: {}", err),
                    }
                },
                Err(err) => println!("Failed to get the response: {:?}", err),
            }
        },

        Check::Alerts(details) => {
            let http_client = Client::new();
            let url = format!("http://api.weatherstack.com/current?access_key=api_key&query={}", details.city);
            let http_result = http_client.get(&url).send();

            match http_result {
                Ok(response) => {
                    let text_result = response.text();
                    match text_result {
                        Ok(text) => {
                            match serde_json::from_str::<Value>(&text) {
                                Ok(json) => {
                                    if let Some(location) = json.get("location") {
                                        if let Some(current) = json.get("current") {
                                            println!("Weather alert for {}", location["name"].as_str().unwrap());
                                            if let Some(wind_speed) = current["wind_speed"].as_i64() {
                                                if wind_speed > 20 {
                                                    println!("Wind speed has exceeded 20 km/h");
                                                } else {
                                                    println!("Wind speed is normal");
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(err) => println!("Failed to get the JSON data: {}", err)
                            }
                        },
                        Err(err) => println!("Failed to get the text: {}", err),
                    }
                },
                Err(err) => println!("Failed to get the response: {:?}", err),
            }
        },

        Check::Compare(details) => {
            let mut data = Vec::new();
            for cities in details.cities.split(", ") {
                data.push(cities)
            }

            println!("Weather Comparison");
            for city in data {
                let http_client = Client::new();
                let url = format!("http://api.weatherstack.com/current?access_key=api_key&query={}", city);
                let http_result = http_client.get(&url).send();

                match http_result {
                    Ok(response) => {
                        let text_result = response.text();
                        match text_result {
                            Ok(text) => {
                                match serde_json::from_str::<Value>(&text) {
                                    Ok(json) => {
                                        if let Some(location) = json.get("location") {
                                            if let Some(current) = json.get("current") {
                                                println!("{}: {}C", location["name"].as_str().unwrap(), current["temperature"]);
                                            }
                                        }
                                    },
                                    Err(err) => println!("Failed to get the JSON data: {}", err)
                                }
                            },
                            Err(err) => println!("Failed to get the text: {}", err),
                        }
                    },
                    Err(err) => println!("Failed to get the response: {:?}", err),
                }
            }
        },
    }
}

