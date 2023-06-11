mod engine;
mod recursive_parser;

use std::collections::HashMap;

use engine::*;
use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let rooms = vec![
        Room::new(0, "Room 1", "room 1 description.").with_exits(HashMap::from([
            (Direction::North, Exit::new(1, "path in north.", false)),
            (Direction::West, Exit::new(2, "way in west.", false)),
        ])),
        Room::new(1, "Room 2", "room 2 description").with_exits(HashMap::from([
            (
                Direction::North,
                Exit::new(3, "there is huge door in north.", true),
            ),
            (Direction::South, Exit::new(0, "there is way back.", false)),
        ])),
        Room::new(2, "Room 3", "room 3 description"),
        Room::new(3, "Room 4", "room 4 description"),
    ];
    let mut game = Game::new(rooms);
    game.print_room_info();
    game.navigate(Direction::North);
    game.navigate(Direction::North);
    game.navigate(Direction::South);
    game.navigate(Direction::West);
    game.navigate(Direction::North);
    // let input = String::from("pick rusty sword");

    // match parse_input(input, &vocabulary) {
    //     Ok(_) => println!("parsed successfully."),
    //     Err(err) => println!("{}", err),
    // }
}
