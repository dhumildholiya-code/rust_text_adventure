use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
    None,
}
impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => Direction::None,
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::West => write!(f, "west"),
            Direction::East => write!(f, "east"),
            Direction::None => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    id: usize,
    name: String,
    description: String,
    items: Vec<usize>,
}
impl Room {
    pub fn new(id: usize, name: &str, description: &str) -> Self {
        Room {
            id,
            name: name.to_string(),
            description: description.to_string(),
            items: Vec::new(),
        }
    }
    pub fn with_items(self, items_str: &str) -> Self {
        let mut items: Vec<usize> = Vec::new();
        if items_str.trim() != "" {
            for item_id in items_str.split(" ") {
                items.push(item_id.parse::<usize>().unwrap());
            }
        }
        Room {
            id: self.id,
            name: self.name,
            description: self.description,
            items,
        }
    }
    pub fn _get_id(&self) -> usize {
        self.id
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_room_with_items_id() {
        let room = Room::new(0, "room_1", "good room 1.").with_items("0 1 2");
        let expected: Vec<usize> = vec![0, 1, 2];
        assert_eq!(room.items, expected);
    }
    #[test]
    fn return_room_with_no_items_if_item_string_is_empty() {
        let room = Room::new(0, "room_1", "good room 1.").with_items("");
        let expected: Vec<usize> = vec![];
        assert_eq!(room.items, expected);
    }
    #[test]
    fn return_room_with_no_items_if_item_string_is_full_whitespace() {
        let room = Room::new(0, "room_1", "good room 1.").with_items("  ");
        let expected: Vec<usize> = vec![];
        assert_eq!(room.items, expected);
    }
}
