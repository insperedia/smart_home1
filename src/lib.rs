
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

    pub fn add_device<D>(&mut self, device: D) where D: Device + 'static   {
        self.devices.insert(device.get_name().to_string(),Box::new(device) );
    }

    pub fn get_devices(&self) -> &HashMap<String, Box<dyn Device>> {
        &self.devices
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
    pub fn new(name: String) -> SmartHouse {
        SmartHouse {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn get_rooms(&self) -> &HashMap<String, Room<dyn Device>> {
        &self.rooms
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_room(&mut self, room: Room<dyn Device>) {
        if self.rooms.contains_key(room.get_name()) {
            panic!("{} already exists", room.get_name());
        }
        self.rooms.insert(room.get_name().to_string(), room);
    }

    pub fn create_report(&self, report: &dyn Report) -> String {
        report.create_report(self)
    }
}


