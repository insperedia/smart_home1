#[cfg(test)]
mod tests {
    use smart_home::device::{Socket, Thermometer};
    use smart_home::report::HtmlReport;
    use smart_home::{Room, SmartHomeError, SmartHouse};

    fn prepare() -> SmartHouse {
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

    #[test]
    fn house_has_rooms() {
        let house = prepare();
        let rooms = house.get_rooms();
        let room = rooms.get("Room1").unwrap();
        assert!(room.get_name().len() > 0);
        assert!(room.get_devices().len() > 0);
    }

    #[test]
    fn check_unique_room_enforcement() {
        let mut house = SmartHouse::new("House1".into());
        let room = Room::new("Room1");
        let room2 = Room::new("Room1");
        let _ = house.add_room(room);
        let result = house.add_room(room2);
        assert!(result.is_err());
        let error = result.err().unwrap();
        match error {
            SmartHomeError::GeneralError => {
                panic!("Should not be run")
            }
            SmartHomeError::RoomExistsError(value) => {
                assert_eq!(value, "Room1")
            }
            SmartHomeError::DeviceExistsError(_) => {
                panic!("Should not be run")
            }
        }
    }

    #[test]
    fn check_unique_device_enforcement() {
        let mut room = Room::new("Room1");
        let _ = room.add_device(Socket {
            power: false,
            consumption: 0,
            name: "Socket1".to_string(),
        });
        let result = room.add_device(Socket {
            power: false,
            consumption: 0,
            name: "Socket1".to_string(),
        });
        assert!(result.is_err());
        let error = result.err().unwrap();
        match error {
            SmartHomeError::GeneralError => {
                panic!("Should not be run")
            }
            SmartHomeError::RoomExistsError(_) => {
                panic!("Should not be run")
            }
            SmartHomeError::DeviceExistsError(value) => {
                assert_eq!("Socket1", value)
            }
        }
    }
}
