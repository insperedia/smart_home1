use smart_home;
use smart_home::{Room, SmartHouse};
use smart_home::report::{TextReport, HtmlReport};
use smart_home::device::{Socket, Thermometer};

fn main() {
    let mut house = SmartHouse::new();
    let mut room = Room::new("Room1");
    let socket = Socket {
        power: false,
        consumption: 0,
        name: "Socket1".to_string(),
    };
    room.add_device(&socket);
    house.add_room(&room);

    let mut room2 = Room::new("Room2");
    let socket = Socket {
        power: false,
        consumption: 0,
        name: "Socket2".to_string(),
    };
    room2.add_device(&socket);
    let themometer = Thermometer {
        temperature: 0.0,
        name: "Thermometer1".to_string(),
    };
    room2.add_device(&themometer);
    house.add_room(&room2);

    let report = house.create_report(&TextReport);
    println!("{}", report);

    let report = house.create_report(&HtmlReport);
    println!("{}", report);
}
