mod engine;
mod recursive_parser;

use std::vec;

use engine::*;
use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let rooms = vec![
        Room::new(0, "Room 1", "room 1 description."),
        Room::new(1, "Room 2", "room 2 description"),
        Room::new(2, "Room 3", "room 3 description"),
        Room::new(3, "Room 4", "room 4 description"),
        Room::new(4, "Room 5", "room 5 description"),
        Room::new(5, "Room 6", "room 6 description"),
    ];
    let exits = vec![
        Exit::new(1, "There is open door in {}.", false),
        Exit::new(2, "Huge locked door in {}.", true),
        Exit::new(4, "In {} open pathway.", false),
        Exit::new(3, "Broken little door in {}.", false),
        Exit::new(5, "There is door to {}.", false),
    ];
    let room_exit_table = vec![
        vec![0, -1, -1, -1],
        vec![2, 0, -1, 1],
        vec![-1, -1, 1, -1],
        vec![-1, -1, -1, 3],
        vec![-1, 2, 3, 4],
        vec![-1, -1, 4, -1],
    ];
    let mut game = Game::new(rooms, exits);
    game.populate_room_exit(room_exit_table);
    game.print_room_info();
    game.navigate(Direction::North);
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
