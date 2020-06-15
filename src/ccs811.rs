use chrono::Local;
use rppal::i2c::I2c;
use ccs811;
use std::sync::{Arc, Mutex};

use crate::data::{AirQuality, Environment};

#[derive(Clone)]
pub struct Ccs811 {
    sensor: Arc<Mutex<ccs811::chip::CCS811>>,
}

impl Ccs811 {
    pub fn new() -> Ccs811 {
        let i2c = I2c::with_bus(1).expect("Couldn't start i2c.");
        Ccs811 {
            sensor: Arc::new(Mutex::new(ccs811::new(i2c, None)))
        }
    }

    pub fn activate(&self) {
        let mut sensor = self.sensor.lock().unwrap();
        match sensor.begin() {
            Ok(_) => match sensor.start(ccs811::MODE::Sec1) {
                Ok(_) => (),
                Err(e) => panic!("Couldn't start css811: {}", e)
            },
            Err(e) => panic!("Couldn't begin ccs811: {}", e)
        }
    }

    pub fn read(&self, env: &Environment) -> Result<AirQuality, String> {
        let mut sensor = self.sensor.lock().unwrap();
        let _ = sensor.set_env_data(env.humidity, env.temperature);
        match sensor.read() {
            Ok(data) => {
                Ok(AirQuality {
                    timestamp: Local::now(),
                    co2: From::from(data.e_co2),
                    tvoc: From::from(data.t_voc),
                    environment: *env,
                })
            },
            Err(e) => {
                Err(format!("Couldn't read data: {}", e))
            }
        }
    }
}
