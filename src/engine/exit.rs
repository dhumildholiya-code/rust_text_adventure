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
            id: id,
            room_id: next_room_id,
            description: description.to_string(),
            locked: locked,
        }
    }
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    pub fn get_room_id(&self) -> usize {
        self.room_id
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}
