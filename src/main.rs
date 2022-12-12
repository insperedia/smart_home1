
use crate::smart_home::{SmartHouse, Room, RoomTrait, Socket, Device, TextReport};

mod smart_home;

fn main() {

    let mut room = Room::new("Room1");
    let socket = Socket{
        power: false,
        consumption: 0,
        name: "Socket1".to_string()
    };
    room.add_device(&socket);
    let mut house = SmartHouse::new();
    house.add_room(&room);

    let report = house.create_report(TextReport);
    println!("{}", report);

}
