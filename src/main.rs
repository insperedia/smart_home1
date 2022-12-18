use smart_home::device::{Socket, Thermometer};
use smart_home::report::{HtmlReport, TextReport};
use smart_home::{Room, SmartHouse};

fn main() {
    let mut house = SmartHouse::new("House1".into());
    let mut room = Room::new("Room1");
    let socket = Socket {
        power: false,
        consumption: 0,
        name: "Socket1".to_string(),
    };
    let _ = room.add_device(socket);
    let _ = house.add_room(room);

    let mut room2 = Room::new("Room2");
    let socket = Socket {
        power: false,
        consumption: 0,
        name: "Socket2".to_string(),
    };
    let _ = room2.add_device(socket);
    let themometer = Thermometer {
        temperature: 0.0,
        name: "Thermometer1".to_string(),
    };
    let _ = room2.add_device(themometer);
    let _ = house.add_room(room2);

    let report = house.create_report(&TextReport);
    println!("{}", report);

    let report = house.create_report(&HtmlReport);
    println!("{}", report);
}
