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
    pub fn has_item(&self, item_id: usize) -> bool {
        self.items.contains(&item_id)
    }
    pub fn get_items(&self) -> Vec<usize> {
        self.items.clone()
    }
    pub fn remove_item(&mut self, item_id: usize) -> Option<usize> {
        match self.items.iter().position(|x| *x == item_id) {
            Some(index) => Some(self.items.swap_remove(index)),
            None => None,
        }
    }
    pub fn add_item(&mut self, item_id: usize) {
        self.items.push(item_id)
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_removed_item_if_item_in_room() {
        let mut room = Room::new(0, "room_1", "good room 1.").with_items("0 1 2");
        let expected_1 = Some(1);
        let expected_2 = Some(2);
        assert_eq!(room.remove_item(1), expected_1);
        assert_eq!(room.remove_item(2), expected_2);
    }
    #[test]
    fn return_none_if_item_not_in_room() {
        let mut room = Room::new(0, "room_1", "good room 1.").with_items("0 1 2");
        assert_eq!(room.remove_item(4), None);
        assert_eq!(room.remove_item(5), None);
    }
    #[test]
    fn check_add_item_adds_an_item_in_room() {
        let mut room = Room::new(0, "room_1", "good room 1.").with_items("0");
        room.add_item(5);
        assert_eq!(room.items[room.items.len() - 1], 5);
        room.add_item(6);
        assert_eq!(room.items[room.items.len() - 1], 6);
    }
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
