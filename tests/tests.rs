#[cfg(test)]
mod tests {
    use smart_home::device::{Socket, Thermometer};
    use smart_home::{Room, SmartHouse};
    use smart_home::report::HtmlReport;

    fn prepare() -> SmartHouse {
        let mut house = SmartHouse::new("House1".into());
        let mut room = Room::new("Room1");
        let socket = Socket {
            power: false,
            consumption: 0,
            name: "Socket1".to_string(),
        };
        room.add_device(socket);
        house.add_room(room);

        let mut room2 = Room::new("Room2");
        let socket = Socket {
            power: false,
            consumption: 0,
            name: "Socket2".to_string(),
        };
        room2.add_device(socket);
        let themometer = Thermometer {
            temperature: 0.0,
            name: "Thermometer1".to_string(),
        };
        room2.add_device(themometer);
        house.add_room(room2);

        house
    }

    #[test]
    fn html_report() {
        let html_report = prepare().create_report(&HtmlReport);
        assert!(html_report.contains("<html>"));
    }

    #[test]
    fn house_has_name_and_rooms() {
        let house = prepare();
        assert!(house.get_name().len() > 0);
        let rooms = house.get_rooms();
        assert!(rooms.len() > 0);
    }


}
