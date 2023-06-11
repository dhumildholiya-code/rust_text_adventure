use crate::Exit;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
#[derive(Debug, Clone)]
pub struct Room {
    id: usize,
    name: String,
    description: String,
    exits: HashMap<Direction, Exit>,
}
impl Room {
    pub fn new(id: usize, name: &str, description: &str) -> Self {
        Room {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
        }
    }
    pub fn _get_id(&self) -> usize {
        self.id
    }
    pub fn get_description(&self) -> String {
        let mut details = self.description.clone() + "\n";
        for (_key, value) in self.exits.iter() {
            details += value.get_description().as_str();
            details += "\n";
        }
        details
    }
    pub fn add_exit(&mut self, direction: Direction, exit: Exit) -> bool {
        if !self.exits.contains_key(&direction) {
            self.exits.insert(direction, exit);
            return true;
        }
        false
    }
    pub fn get_next_room_id(&self, direction: Direction) -> Option<usize> {
        match self.exits.get(&direction) {
            Some(exit) => Some(exit.get_room_id()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exit_true_if_key_not_exists() {
        let exit = Exit::new(1, "test exit north", false);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        assert_eq!(true, room.add_exit(Direction::North, exit));
    }
    #[test]
    fn add_exit_false_if_key_exists() {
        let exit = Exit::new(1, "test exit north", false);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        room.add_exit(Direction::North, exit.clone());
        assert_eq!(false, room.add_exit(Direction::North, exit));
    }
    #[test]
    fn get_room_id_if_exit_in_direction_exist() {
        let exit1 = Exit::new(1, "test exit north", false);
        let exit2 = Exit::new(2, "test exit south", false);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        room.add_exit(Direction::North, exit1.clone());
        room.add_exit(Direction::South, exit2.clone());
        assert_eq!(
            room.get_next_room_id(Direction::North),
            Some(exit1.get_room_id())
        );
        assert_eq!(
            room.get_next_room_id(Direction::South),
            Some(exit2.get_room_id())
        );
    }
    #[test]
    fn get_none_if_exit_for_direction_not_exist() {
        let room = Room::new(0, "test room", "Testing Room Description.");
        assert_eq!(room.get_next_room_id(Direction::North).is_none(), true);
        assert_eq!(room.get_next_room_id(Direction::South).is_none(), true);
    }
    #[test]
    fn description_without_exits_in_room() {
        let expected = "Testing Room Description.\n";
        let room = Room::new(0, "test room", "Testing Room Description.");
        assert_eq!(expected.to_string(), room.get_description());
    }
    #[test]
    fn description_with_exits_in_room() {
        let expected1 = "Testing Room Description.\ntest exit north\ntest exit south\n";
        let expected2 = "Testing Room Description.\ntest exit south\ntest exit north\n";
        let exit1 = Exit::new(1, "test exit north", false);
        let exit2 = Exit::new(2, "test exit south", false);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        room.add_exit(Direction::North, exit1);
        room.add_exit(Direction::South, exit2);
        let result = room.get_description();
        let is_true = (result == expected1) || (result == expected2);
        assert_eq!(is_true, true);
    }
}
