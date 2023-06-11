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
    pub fn navigate(&mut self, direction: Direction) {
        match self.rooms[self.current_room_id].get_next_room_id(direction) {
            room::ExitRoomId::Id(next_room_id) => self.change_room(next_room_id),
            room::ExitRoomId::Locked(info) => self.response(info),
            room::ExitRoomId::NoExit => self.response("There is no path.".to_string()),
        }
    }
    pub fn print_room_info(&self) {
        println!("{}", self.rooms[self.current_room_id].get_description());
    }
    fn change_room(&mut self, new_room_id: usize) {
        if self.current_room_id != new_room_id && new_room_id < self.rooms.len() {
            self.current_room_id = new_room_id;
            self.response(self.rooms[self.current_room_id].get_description());
        }
    }
    fn response(&self, text: String) {
        println!("{}", text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
