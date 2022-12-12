#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let home = SmartHome::new();
        let room = SmartHome::new_room("room1");
        room.add(Socket);
        room.add(Thermometer);
        let room = SmartHome::new_room("room2");
        room.add(Thermometer);

        let html_report = home.create_report(SmartHomeHtmlReport);
        assert_eq!(html_report, "<html><room><title>Room1</title><ul><li>Socket (power: On)</li><li>Thermometer (Themperature: 24)</li></ul></room><room><title>Room1</title><ul><li>Socket (power: Off)</li></ul></room></html>")
    }
}