use crate::{Item, Player};

pub fn list_inventory(items: &Vec<Item>, player: &Player) -> String {
    let len = player.get_inventory().len();
    let mut content:String = match len {
        0 => "You have no items in bag.".to_string(),
        1 => "You have 1 item in bag".to_string(),
        _ => format!("You have {} items in bag.", len),
    };
    for item_id in player.get_inventory() {
        content += format!("\n- {}", items[*item_id].get_name()).as_str();
    }
    println!("\n{}", content);
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
    }
    #[test]
    fn check_list_inventory_with_1_item_generates_proper_response() {
        let mut player = test_player();
        player.add_item(1);
        let items = test_items();

        let result = list_inventory(&items, &player);

        let expected = "You have 1 item in bag.\n- stick";
    }
    #[test]
    fn check_list_inventory_with_n_items_generates_proper_response() {
        let mut player = test_player();
        player.add_item(1);
        player.add_item(2);
        let items = test_items();

        let result = list_inventory(&items, &player);

        let expected = "you have 2 items.\n- stick\n- sword";
    }
    fn test_player() -> Player {
        Player::new(100, 100)
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
