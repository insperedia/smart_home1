
pub mod device;
pub mod report;
use std::collections::HashMap;
use crate::device::Device;
use crate::report::Report;

#[allow(dead_code)]
#[derive(Debug)]
pub enum SmartHomeError {
    GeneralError,
}

pub struct Room<'a> {
    name: String,
    devices: HashMap<String, &'a dyn Device>,
}

impl<'a> Room<'a> {
    pub fn new(name: &str) -> Room {
        Room {
            name: name.into(),
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: &'a dyn Device) {
        self.devices.insert(device.get_name().to_string(), device);
    }

    pub fn get_devices(&self) -> &HashMap<String, &'a dyn Device> {
        &self.devices
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct SmartHouse<'a> {
    rooms: HashMap<&'a str, &'a Room<'a>>,
}

impl<'a> SmartHouse<'a> {
    pub fn new() -> SmartHouse<'a> {
        SmartHouse {
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> &HashMap<&'a str, &'a Room<'a>> {
        &self.rooms
    }

    pub fn add_room(&mut self, room: &'a Room) {
        if self.rooms.contains_key(room.get_name()) {
            panic!("{} already exists", room.get_name());
        }
        self.rooms.insert(room.get_name(), room);
    }

    pub fn create_report(&self, report: &dyn Report) -> String {
        report.create_report(self)
    }
}


