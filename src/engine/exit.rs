use crate::Direction;

#[derive(Debug, PartialEq)]
pub enum ExitResult {
    Id(usize),
    Locked(String),
    NoExit,
}
#[derive(Debug, Clone)]
pub struct Exit {
    id: usize,
    room_id: usize,
    description: String,
    locked: bool,
}
impl Exit {
    pub fn new(id: usize, next_room_id: usize, description: &str, locked: bool) -> Self {
        Exit {
            id,
            room_id: next_room_id,
            description: description.to_string(),
            locked,
        }
    }
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    pub fn get_room_id(&self) -> usize {
        self.room_id
    }
    pub fn get_description(&self, direction: i32) -> String {
        let dir_enum: Direction = direction.into();
        self.description
            .replace("{}", dir_enum.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn return_exit_description_according_to_direction() {
        let exit = Exit::new(0, 1, "Door in {}.", false);
        let expected_north = "Door in north.";
        let expected_south = "Door in south.";
        let expected_west = "Door in west.";
        let expected_east = "Door in east.";

        assert_eq!(exit.get_description(0), expected_north);
        assert_eq!(exit.get_description(1), expected_south);
        assert_eq!(exit.get_description(2), expected_west);
        assert_eq!(exit.get_description(3), expected_east);
    }
}
