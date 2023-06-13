pub struct Item {
    id: usize,
    name: String,
    room_position: String,
    description: String,
}
impl Item {
    pub fn new(id: usize, name: &str, room_desc: &str, description: &str) -> Self {
        Item {
            id: id,
            name: name.to_string(),
            room_position: room_desc.replace("{name}", name),
            description: description.replace("{name}", name),
        }
    }
    pub fn get_room_position(&self) -> String {
        self.room_position.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_item_room_position_description() {
        let item = Item::new(0, "item", "{name} lying on ground.", "desc");
        let expected = "item lying on ground.";
        assert_eq!(item.get_room_position(), expected);
    }
    #[test]
    fn testing_item_description() {
        let item = Item::new(0, "item", "{name} lying", "{name} is a god key.");
        let expected = "item is a god key.";
        assert_eq!(item.get_description(), expected);
    }
}
