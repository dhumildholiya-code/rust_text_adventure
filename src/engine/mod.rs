mod exit;
mod room;

pub use exit::Exit;
pub use room::{Direction, Room};

#[derive(Debug)]
pub struct Game {
    rooms: Vec<Room>,
    current_room_id: usize,
}
impl Game {
    pub fn new(rooms: Vec<Room>) -> Self {
        Game {
            rooms: rooms,
            current_room_id: 0,
        }
    }
    pub fn change_room(&mut self, new_room_id: usize) -> bool {
        if self.current_room_id != new_room_id && new_room_id < self.rooms.len() {
            self.current_room_id = new_room_id;
            self.response(self.rooms[self.current_room_id].get_description());
            return true;
        }
        false
    }
    pub fn response(&self, text: String) {
        println!("{}", text);
    }
    pub fn print_room_info(&self) {
        println!("{}", self.rooms[self.current_room_id].get_description());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn change_room_true_if_rooms_available() {
        let rooms = vec![
            Room::new(0, "name", "room 1 description").with_exits(HashMap::from([
                (Direction::North, Exit::new(1, "description", false)),
                (Direction::West, Exit::new(2, "description", false)),
            ])),
            Room::new(1, "name", "room 2 description").with_exits(HashMap::from([
                (Direction::North, Exit::new(1, "description", false)),
                (Direction::South, Exit::new(3, "description", false)),
            ])),
            Room::new(2, "name", "room 3 description"),
            Room::new(3, "name", "room 4 description"),
        ];
        let mut game = Game::new(rooms);
        assert_eq!(game.change_room(2), true);
    }
    #[test]
    fn change_room_false_if_rooms_not_available() {
        let rooms = vec![
            Room::new(0, "name", "room 1 description").with_exits(HashMap::from([
                (Direction::North, Exit::new(1, "description", false)),
                (Direction::West, Exit::new(2, "description", false)),
            ])),
            Room::new(1, "name", "room 2 description").with_exits(HashMap::from([
                (Direction::North, Exit::new(1, "description", false)),
                (Direction::South, Exit::new(3, "description", false)),
            ])),
            Room::new(2, "name", "room 3 description"),
            Room::new(3, "name", "room 4 description"),
        ];
        let mut game = Game::new(rooms);
        assert_eq!(game.change_room(4), false);
    }
}
