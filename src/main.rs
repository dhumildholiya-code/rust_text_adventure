mod engine;
mod recursive_parser;

use std::vec;

use engine::*;
use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let rooms = vec![
        Room::new(0, "Room 1", "room 1 description.").with_items("1"),
        Room::new(1, "Room 2", "room 2 description").with_items("0"),
        Room::new(2, "Room 3", "room 3 description"),
        Room::new(3, "Room 4", "room 4 description"),
        Room::new(4, "Room 5", "room 5 description"),
        Room::new(5, "Room 6", "room 6 description"),
    ];
    let exits = vec![
        Exit::new(0, 1, "There is open door in {}.", false),
        Exit::new(1, 2, "Huge locked door in {}.", true),
        Exit::new(2, 4, "In {} open pathway.", false),
        Exit::new(3, 3, "Broken little door in {}.", false),
        Exit::new(4, 5, "There is door to {}.", false),
    ];
    let items = vec![
        Item::new(
            0,
            "key",
            "There is a {name} on floor.",
            "{name} is god key.",
        ),
        Item::new(
            1,
            "stick",
            "A Long {name} is in room.",
            "Its deadly weapon.",
        ),
    ];
    let room_exit_table = vec![
        vec![0, -1, -1, -1],
        vec![2, 0, -1, 1],
        vec![-1, -1, 1, -1],
        vec![-1, -1, -1, 3],
        vec![-1, 2, 3, 4],
        vec![-1, -1, 4, -1],
    ];

    let mut game = Game::new(rooms, exits, items);
    game.populate_room_exit(room_exit_table);
    game.response(game.get_room_info(0));
    game.inspect_item("key");
    game.inspect_item("stick");
    game.inspect_item("sti");
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
