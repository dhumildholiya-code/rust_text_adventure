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
}
