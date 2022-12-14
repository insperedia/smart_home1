use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub enum SmartHomeError {
    GeneralError,
}

pub trait Device {
    fn get_info(&self) -> HashMap<String, String>;
    fn get_name(&self) -> &str;
}

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

#[allow(dead_code)]
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

pub trait Report {
    fn create_report(&self, smart_house: &SmartHouse) -> String;
}

pub struct TextReport;

impl Report for TextReport {
    fn create_report(&self, smart_house: &SmartHouse) -> String {
        let mut result = String::from("Text report:\n");
        let rooms = &smart_house.rooms;

        for (room_name, room_trait) in rooms {
            result.push_str(format!("\tRoom: {room_name}\n").as_str());
            for device_name in room_trait.get_devices().keys() {
                result.push_str(format!("\t\t{device_name}\n").as_str());
            }
        }
        result
    }
}

pub struct HtmlReport;

impl Report for HtmlReport {
    fn create_report(&self, smart_house: &SmartHouse) -> String {
        let mut result = String::from("<html><body><h1>Text report</h1>\n");
        let rooms = &smart_house.rooms;

        for (room_name, room_trait) in rooms {
            result.push_str("<div>");
            result.push_str(format!("\t<b>Room: {room_name}<b><br/>").as_str());
            for device_name in room_trait.get_devices().keys() {
                result.push_str(format!("\t\t{device_name}<br/>").as_str());
            }
            result.push_str("</div>");
        }
        result.push_str("</body></html>");
        result
    }
}
