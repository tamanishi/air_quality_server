use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Environment {
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AirQuality {
    pub timestamp: DateTime<Local>,
    pub co2: f32,
    pub tvoc: f32,
    pub environment: Environment,
}
