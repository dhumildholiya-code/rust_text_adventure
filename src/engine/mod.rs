mod exit;
mod item;
mod room;

pub use exit::{Exit, ExitResult};
pub use item::Item;
pub use room::{Direction, Room};

#[derive(Debug)]
pub struct Game {
    rooms: Vec<Room>,
    exits: Vec<Exit>,
    items: Vec<Item>,
    room_exit: Vec<Vec<i32>>,
    current_room_id: usize,
}
impl Game {
    pub fn new(rooms: Vec<Room>, exits: Vec<Exit>, items: Vec<Item>) -> Self {
        let room_len = rooms.len();
        Game {
            rooms,
            exits,
            items,
            room_exit: vec![vec![-1; 4]; room_len],
            current_room_id: 0,
        }
    }
    pub fn populate_room_exit(&mut self, data: Vec<Vec<i32>>) {
        self.room_exit = data;
    }
    pub fn navigate(&mut self, direction: Direction) {
        match self.get_next_room_id(direction) {
            ExitResult::Id(next_room_id) => self.change_room(next_room_id),
            ExitResult::Locked(info) => self.response(info),
            ExitResult::NoExit => self.response(format!("There is no path in {}", direction)),
        }
    }
    pub fn get_room_info(&self, room_id: usize) -> String {
        let mut content = self.rooms[room_id].get_description();
        content += "\n";
        // adding exit info in room description.
        for (id, exit_id) in self.room_exit[room_id].iter().enumerate() {
            if *exit_id != -1 {
                let e_id = *exit_id as usize;
                content += self.exits[e_id].get_description(id as i32).as_str();
                content += " ";
            }
        }
        //adding item info in room description.
        let items_in_room = self.rooms[room_id].get_items();
        if items_in_room.len() > 0 {
            content += "\n";
            for item_id in items_in_room {
                content += self.items[item_id].get_room_position().as_str();
                content += " ";
            }
        }
        content
    }
    fn get_next_room_id(&self, direction: Direction) -> ExitResult {
        if self.room_exit[self.current_room_id][direction as usize] == -1 {
            return ExitResult::NoExit;
        } else {
            let exit_id: usize = self.room_exit[self.current_room_id][direction as usize] as usize;
            if self.exits[exit_id].is_locked() {
                return ExitResult::Locked("Door is locked".to_string());
            } else {
                return ExitResult::Id(self.exits[exit_id].get_room_id());
            }
        }
    }
    fn change_room(&mut self, new_room_id: usize) {
        if new_room_id < self.rooms.len() {
            self.current_room_id = new_room_id;
            self.response(self.get_room_info(self.current_room_id));
        }
    }
    pub fn response(&self, text: String) {
        println!("\n{}\n", text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_room_if_exit_exist() {
        let mut game = test_game();
        game.navigate(Direction::North);
        assert_eq!(game.current_room_id, 1);
        game.navigate(Direction::North);
        assert_eq!(game.current_room_id, 4);
        game.navigate(Direction::West);
        assert_eq!(game.current_room_id, 3);
    }
    #[test]
    fn does_not_change_room_if_exit_locked() {
        let mut game = test_game();
        game.navigate(Direction::North);
        game.navigate(Direction::East);
        assert_eq!(game.current_room_id, 1);
    }
    #[test]
    fn does_not_change_room_if_exit_not_exist() {
        let mut game = test_game();
        game.navigate(Direction::North);
        game.navigate(Direction::West);
        assert_eq!(game.current_room_id, 1);
    }
    #[test]
    fn get_exitid_if_exit_is_unlocked() {
        let game = test_game();
        let expected = ExitResult::Id(1);
        let result = game.get_next_room_id(Direction::North);
        assert_eq!(result, expected);
    }
    #[test]
    fn get_locked_if_exit_is_locked() {
        let mut game = test_game();
        game.change_room(1);
        let expected = ExitResult::Locked("Door is locked".to_string());
        let result = game.get_next_room_id(Direction::East);
        assert_eq!(result, expected);
    }
    #[test]
    fn get_no_exit_if_exit_not_exist() {
        let game = test_game();
        let expected = ExitResult::NoExit;
        let result = game.get_next_room_id(Direction::East);
        assert_eq!(result, expected);
    }

    #[test]
    fn check_room_description_is_valid() {
        let game = test_game();
        let expected_1 = "room 1 description.\nThere is open door in north. 
A Long stick is in room. ";
        let expected_2 = "room 2 description.\nAn open pathway in north. There is open door in south. There is huge locked door in east. 
There is a key on floor. ";
        assert_eq!(game.get_room_info(0), expected_1);
        assert_eq!(game.get_room_info(1), expected_2);
    }

    fn test_game() -> Game {
        let rooms = vec![
            Room::new(0, "Room 1", "room 1 description.").with_items("1"),
            Room::new(1, "Room 2", "room 2 description.").with_items("0"),
            Room::new(2, "Room 3", "room 3 description."),
            Room::new(3, "Room 4", "room 4 description."),
            Room::new(4, "Room 5", "room 5 description."),
            Room::new(5, "Room 6", "room 6 description."),
        ];
        let exits = vec![
            Exit::new(0, 1, "There is open door in {}.", false),
            Exit::new(1, 2, "There is huge locked door in {}.", true),
            Exit::new(2, 4, "An open pathway in {}.", false),
            Exit::new(3, 3, "A little door in {}.", false),
            Exit::new(4, 5, "In {} there is an open door.", false),
        ];
        let items = vec![
            Item::new(
                0,
                "key",
                "There is a {name} on floor.",
                "{name} is god key.",
            ),
            Item::new(
                1,
                "stick",
                "A Long {name} is in room.",
                "Its deadly weapon.",
            ),
        ];
        let room_exit_table = vec![
            vec![0, -1, -1, -1],
            vec![2, 0, -1, 1],
            vec![-1, -1, 1, -1],
            vec![-1, -1, -1, 3],
            vec![-1, 2, 3, 4],
            vec![-1, -1, 4, -1],
        ];
        let mut game = Game::new(rooms, exits, items);
        game.populate_room_exit(room_exit_table);
        game
    }
}
