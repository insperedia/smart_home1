pub mod device;
pub mod report;

use serde::{Serialize, Deserialize};
use crate::device::Device;
use crate::report::Report;
use std::collections::HashMap;
use thiserror::Error;
use serde::Serializer;


#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum SmartHomeError {
    #[error("General error")]
    GeneralError,
    #[error("Room {0} already exists")]
    RoomExistsError(String),
    #[error("Device {0} already exists")]
    DeviceExistsError(String),
    #[error("Room {0} not exists")]
    RoomNotExistsError(String),
    #[error("Device {0} not exists")]
    DeviceNotExistsError(String),
}

#[derive(Serialize, Deserialize)]
pub struct Room<D: ?Sized> {
    name: String,
    devices: HashMap<String, Box<D>>,
}

impl Room<dyn Device> {
    pub fn new(name: &str) -> Room<dyn Device> {
        Room {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device<D>(&mut self, device: D) -> Result<bool, SmartHomeError>
        where
            D: Device + 'static,
    {
        if self.devices.contains_key(device.get_name()) {
            return Err(SmartHomeError::DeviceExistsError(
                device.get_name().to_string(),
            ));
        }
        self.devices
            .insert(device.get_name().to_string(), Box::new(device));
        Ok(true)
    }

    pub fn get_devices(&self) -> &HashMap<String, Box<dyn Device>> {
        &self.devices
    }

    pub fn remove_device(&mut self, device_name: &str) -> Result<(), SmartHomeError> {
        let device_name = device_name.trim();
        if self.devices.contains_key(device_name) {
            self.devices.remove(device_name);
            Ok(())
        } else {
            Err(SmartHomeError::DeviceNotExistsError(device_name.to_string()))
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room<dyn Device>>,
}

impl SmartHouse {
    pub fn remove_room(&mut self, room: &str) -> Result<(), SmartHomeError> {
        let room = room.trim();
        if self.rooms.contains_key(room) {
            self.rooms.remove(room);
            return Ok(());
        } else {
            return Err(SmartHomeError::RoomNotExistsError(room.to_string()));
        }
    }
}

impl SmartHouse {
    pub fn new(name: String) -> SmartHouse {
        SmartHouse {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> &HashMap<String, Room<dyn Device>> {
        &self.rooms
    }

    pub fn get_rooms_mut(&mut self) -> &mut HashMap<String, Room<dyn Device>> {
        &mut self.rooms
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_room(&mut self, room: Room<dyn Device>) -> Result<bool, SmartHomeError> {
        if self.rooms.contains_key(room.get_name()) {
            return Err(SmartHomeError::RoomExistsError(room.get_name().to_string()));
        }
        self.rooms.insert(room.get_name().to_string(), room);
        Ok(true)
    }

    pub fn create_report(&self, report: &dyn Report) -> String {
        report.create_report(self)
    }
}
