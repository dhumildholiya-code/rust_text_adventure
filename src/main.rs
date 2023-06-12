mod engine;
mod recursive_parser;

use std::collections::HashMap;

use engine::*;
use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let rooms = vec![
        Room::new(0, "Room 1", "room 1 description."),
        Room::new(1, "Room 2", "room 2 description"),
        Room::new(2, "Room 3", "room 3 description"),
        Room::new(3, "Room 4", "room 4 description"),
    ];
    let mut game = Game::new(rooms);
    game.print_room_info();
    // game.navigate(Direction::North);
    // game.navigate(Direction::North);
    // game.navigate(Direction::South);
    // game.navigate(Direction::West);
    // game.navigate(Direction::North);
    // let input = String::from("pick rusty sword");

    // match parse_input(input, &vocabulary) {
    //     Ok(_) => println!("parsed successfully."),
    //     Err(err) => println!("{}", err),
    // }
}
