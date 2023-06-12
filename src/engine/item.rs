pub struct Item {
    id: usize,
    name: String,
    room_desc: String,
    description: String,
}
impl Item {
    pub fn new(id: usize, name: &str, room_desc: &str, description: &str) -> Self {
        Item {
            id: id,
            name: name.to_string(),
            room_desc: room_desc.replace("{name}", name),
            description: description.replace("{name}", name),
        }
    }
    pub fn get_room_desc(&self) -> String {
        self.room_desc.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_desc() {
        let item = Item::new(0, "test item", "{name} is lying on ground.", "its god key.");
        let result = item.get_room_desc();
        let expected = "test item is lying on gronud.";
        assert_eq!(result.as_str(), expected);
    }
}
