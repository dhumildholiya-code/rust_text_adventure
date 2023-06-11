mod engine;
mod recursive_parser;

use engine::*;
use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();

    let mut rooms = vec![
        Room::new(0, "Room 1", "crazy room."),
        Room::new(1, "Room 1", "more crazy room."),
        Room::new(2, "Room 1", "are u stupid."),
    ];
    rooms[0].add_exit(Direction::North, Exit::new(1, "exit in north", false));

    let mut current_room_id = 0;
    println!("{}", rooms[current_room_id].get_description());

    match rooms[current_room_id].get_next_room_id(Direction::North) {
        Some(id) => current_room_id = id,
        None => println!("No exit there!"),
    }

    println!("{}", rooms[current_room_id].get_description());

    // let input = String::from("pick rusty sword");

    // match parse_input(input, &vocabulary) {
    //     Ok(_) => println!("parsed successfully."),
    //     Err(err) => println!("{}", err),
    // }
}
