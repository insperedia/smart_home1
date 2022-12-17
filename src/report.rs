use crate::SmartHouse;

pub trait Report {
    fn create_report(&self, smart_house: &SmartHouse) -> String;
}

pub struct TextReport;
impl Report for TextReport {
    fn create_report(&self, smart_house: &SmartHouse) -> String {
        let mut result = String::from("Text report:\n");
        let rooms = smart_house.get_rooms();

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
        let rooms = smart_house.get_rooms();

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
