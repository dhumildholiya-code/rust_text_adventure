use crate::engine::{Item, Player, Room};

pub fn list_inventory(items: &Vec<Item>, player: &Player) -> String {
    let len = player.get_inventory().len();
    let mut content: String = match len {
        0 => "You have no items in bag.".to_string(),
        1 => "You have 1 item in bag.".to_string(),
        _ => format!("You have {} items in bag.", len),
    };
    for item_id in player.get_inventory() {
        content += format!("\n- {}", items[*item_id].get_name()).as_str();
    }
    println!("\n{}", content);
    content
}
pub fn drop_item(
    item_name: &str,
    items: &Vec<Item>,
    room: &mut Room,
    player: &mut Player,
) -> String {
    let mut content = "".to_string();
    match items.iter().position(|x| x.get_name() == item_name) {
        Some(item_id) => match player.remove_item(item_id) {
            Some(removed_id) => {
                room.add_item(removed_id);
                content += format!("You dropped {} item.", item_name).as_str();
            }
            None => content += "No item like that in your bag.",
        },
        None => content += format!("There is no item named {}", item_name).as_str(),
    }
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_list_inventory_with_0_items_generates_proper_response() {
        let mut player = test_player();
        let items = test_items();

        let result = list_inventory(&items, &player);

        let expected = "You have no items in bag.";
        assert_eq!(result, expected);
    }
    #[test]
    fn check_list_inventory_with_1_item_generates_proper_response() {
        let mut player = test_player();
        player.add_item(1);
        let items = test_items();

        let result = list_inventory(&items, &player);

        let expected = "You have 1 item in bag.\n- stick";
        assert_eq!(result, expected);
    }
    #[test]
    fn check_list_inventory_with_n_items_generates_proper_response() {
        let mut player = test_player();
        player.add_item(1);
        player.add_item(2);
        let items = test_items();

        let result = list_inventory(&items, &player);

        let expected = "You have 2 items in bag.\n- stick\n- sword";
        assert_eq!(result, expected);
    }
    #[test]
    fn drops_item_if_you_have_item_in_your_bag() {
        let mut player = test_player();
        player.add_item(3);
        let mut room = test_room();
        let items = test_items();

        let result = drop_item("box", &items, &mut room, &mut player);

        let expected = "You dropped box item.";
        assert_eq!(result, expected);
        assert_eq!(player.get_inventory().len(), 0);
        assert_eq!(room.get_items().len(), 3);
    }
    #[test]
    fn does_not_drops_item_if_you_does_not_have_item_in_your_bag() {
        let mut player = test_player();
        player.add_item(1);
        let mut room = test_room();
        let items = test_items();

        let result = drop_item("sword", &items, &mut room, &mut player);

        let expected = "No item like that in your bag.";
        assert_eq!(result, expected);
        assert_eq!(player.get_inventory().len(), 1);
        assert_eq!(room.get_items().len(), 2);
    }
    fn test_player() -> Player {
        Player::new(100, 100)
    }
    fn test_room() -> Room {
        Room::new(0, "Room 1", "room 1 description.").with_items("1 2")
    }
    fn test_items() -> Vec<Item> {
        vec![
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
            Item::new(
                2,
                "sword",
                "A Long {name} is in the corner of the room.",
                "Its deadly weapon.",
            ),
            Item::new(
                3,
                "box",
                "There is {name} in the left corner.",
                "Its deadly weapon.",
            ),
        ]
    }
}
