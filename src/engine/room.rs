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
    fn from(value: i32) -> Self{
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
}
impl Room {
    pub fn new(id: usize, name: &str, description: &str) -> Self {
        Room {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
        }
    }
    pub fn _get_id(&self) -> usize {
        self.id
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}
