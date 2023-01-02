use smart_home::device::{Device, Socket, Thermometer};
use smart_home::report::{HtmlReport, TextReport};
use smart_home::{Room, SmartHomeError, SmartHouse};
use std::{io, process};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::stdout;


fn main() {
    let mut house = SmartHouse::new("House1".into());
    io::stdout().write_all("Добро пожаловать в программу управления умным домом.\n".as_bytes()).unwrap();
    loop {
        io::stdout().write_all("Введите команду (add,view,delete,report,exit):\n".as_bytes()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.as_str().trim() {
            "exit" => process::exit(0),
            "add" => add_cmd(&mut house),
            "view" => list_house(&house),
            "report" => report(&house),
            "delete" => delete_cmd(&mut house),
            _ => { io::stdout().write_all("Не известная команда\n".as_bytes()).unwrap(); }
        }
    }

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

    let room = Room::new("Room1");
    let result = house.add_room(room);
    if result.is_err()
    {
        println!("{}", result.unwrap_err());
    }
}

fn delete_cmd(house: &mut SmartHouse) {
    stdout().write_all("Введите название комнаты для удаления или команата>устройство\n".as_bytes()).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.contains(">") {
        let input = input.split_once(">").unwrap();
        let room_result = house.get_rooms_mut().get_mut(input.0);
        match room_result  {
            None => {
                println!("Комната {} не существует", input.0);
            }
            Some(room) => {
                let devices = room.remove_device(input.1);
                match devices {
                    Ok(_) => {
                        println!("Устройство {} удалено", input.1);
                    }
                    Err(err) => if let SmartHomeError::DeviceNotExistsError(string) = err {
                        println!("Устройство {} не существует", input.1);
                    } else {
                        println!("Не известная ошибка");
                    }

                }
            }
        }
    } else {
       let result = house.remove_room(input.as_str());
       match result {
           Ok(_) => {println!("Комната {} удалена", input)}
           Err(error_string) => {println!("{}", error_string)}
       }
    }
}

fn report(house: &SmartHouse) {
    let report = house.create_report(&TextReport);
    println!("{}", report);
}

fn list_house(house: &SmartHouse) {
    for room in house.get_rooms() {
        stdout().write_all(format!("Room {}\n", room.0).as_bytes()).unwrap();
        for device in room.1.get_devices() {
            stdout().write_all(format!("    {}\n", device.0).as_bytes()).unwrap();
        }
    }
}

fn add_cmd(house: &mut SmartHouse) {
    loop {
        stdout().write_all("Введите тип (room, socket, thermometer) и название или введите exit:\n".as_bytes()).unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_string = input.as_str().trim();
        if input_string == "exit" {
            return;
        }
        let input_string = input_string.split_once(' ');
        let input_string = match input_string {
            None => {
                stdout().write_all("Неверный вводод. Введите тип и назавание.\n".as_bytes()).unwrap();
                continue;
            }
            Some(val) => val
        };
        match input_string.0 {
            "room" => {
                let room = Room::new(input_string.1);
                let _ = house.add_room(room);
                stdout().write_all("Команата добавлена\n".as_bytes()).unwrap();
            }
            "socket" => {
                let room = get_room(house);
                match room {
                    None => { stdout().write_all("Комната не найдена.\n".as_bytes()).unwrap(); }
                    Some(room) => {
                        let socket = Socket {
                            name: input_string.1.to_string(),
                            ..Default::default()
                        };
                        room.add_device(socket).unwrap();
                    }
                }
            }
            "thermometer" => {
                let room = get_room(house);
                match room {
                    None => { stdout().write_all("Комната не найдена.\n".as_bytes()).unwrap(); }
                    Some(room) => {
                        let dev = Thermometer {
                            name: input_string.1.to_string(),
                            ..Default::default()
                        };
                        room.add_device(dev).unwrap();
                    }
                }
            }
            _ => { stdout().write_all("Не известная команда\n".as_bytes()).unwrap(); }
        }
    }
}

fn get_room(house: &mut SmartHouse) -> Option<&mut Room<dyn Device>> {
    if house.get_rooms().len() == 0 {
        return Option::None;
    }

    stdout().write_all("Веберите комнату (Введите название):\n".as_bytes()).unwrap();

    for room in house.get_rooms() {
        stdout().write_all(format!("{}\n", room.0).as_bytes()).unwrap();
    };

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let result = house.get_rooms_mut().get_mut(input.as_str().trim());
    match result {
        None => {
            stdout().write_all(format!("Комната {} не найдена\n", input).as_bytes()).unwrap();
            return Option::None;
        }
        Some(val) => {
            return Option::Some(val);
        }
    }
}
