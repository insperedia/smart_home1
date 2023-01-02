use crate::SmartHomeError;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub trait Device {
    fn get_info(&self) -> HashMap<String, String>;
    fn get_name(&self) -> &str;
}

#[derive(Default)]
pub struct Socket {
    pub power: bool,
    pub consumption: u64,
    pub name: String,
}

#[allow(dead_code)]
impl Socket {
    pub fn get_power() -> bool {
        todo!()
    }
    pub fn get_consumption() -> u64 {
        todo!()
    }
    pub fn set_power(_is_on: bool) -> Result<bool, SmartHomeError> {
        todo!()
    }
}

impl Device for Socket {
    fn get_info(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
#[derive(Default)]
pub struct Thermometer {
    pub temperature: f32,
    pub name: String,
}

#[allow(dead_code)]
impl Thermometer {
    pub fn get_temperature() -> f32 {
        todo!()
    }
}

impl Device for Thermometer {
    fn get_info(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
