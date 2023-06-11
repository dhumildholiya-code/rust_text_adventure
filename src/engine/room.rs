use crate::Exit;
use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::West => write!(f, "west"),
            Direction::East => write!(f, "east"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExitRoomId {
    Id(usize),
    Locked(String),
    NoExit,
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
    pub fn with_exits(self, exits: HashMap<Direction, Exit>) -> Self {
        Room {
            id: self.id,
            name: self.name,
            description: self.description,
            exits: exits,
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
    pub fn get_next_room_id(&self, direction: Direction) -> ExitRoomId {
        match self.exits.get(&direction) {
            Some(exit) => {
                if exit.is_locked() {
                    // TODO: add required items to unlock info later.
                    return ExitRoomId::Locked("Path is locked".to_string());
                } else {
                    return ExitRoomId::Id(exit.get_room_id());
                }
            }
            None => ExitRoomId::NoExit,
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
    fn get_room_id_if_exit_exist_and_not_locked() {
        let exit1 = Exit::new(1, "test exit north", false);
        let exit2 = Exit::new(2, "test exit south", false);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        room.add_exit(Direction::North, exit1.clone());
        room.add_exit(Direction::South, exit2.clone());
        assert_eq!(room.get_next_room_id(Direction::North), ExitRoomId::Id(1),);
        assert_eq!(room.get_next_room_id(Direction::South), ExitRoomId::Id(2));
    }
    #[test]
    fn get_Locked_if_exit_exist_and_locked() {
        let exit1 = Exit::new(1, "test exit north", true);
        let exit2 = Exit::new(2, "test exit south", true);
        let mut room = Room::new(0, "test room", "Testing Room Description.");
        room.add_exit(Direction::North, exit1.clone());
        room.add_exit(Direction::South, exit2.clone());
        assert_eq!(
            room.get_next_room_id(Direction::North),
            ExitRoomId::Locked("Path is locked".to_string()),
        );
        assert_eq!(
            room.get_next_room_id(Direction::South),
            ExitRoomId::Locked("Path is locked".to_string())
        );
    }
    #[test]
    fn get_NoExit_if_exit_not_exist() {
        let room = Room::new(0, "test room", "Testing Room Description.");
        assert_eq!(room.get_next_room_id(Direction::North), ExitRoomId::NoExit);
        assert_eq!(room.get_next_room_id(Direction::South), ExitRoomId::NoExit);
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
