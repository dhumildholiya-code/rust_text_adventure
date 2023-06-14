#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    money: i32,
    health: u32,
    items: Vec<usize>,
}

impl Player {
    pub fn new(money: i32, health: u32) -> Self {
        Player {
            name: "".to_string(),
            money: money,
            health: health,
            items: Vec::new(),
        }
    }
    pub fn get_inventory(&self)-> &Vec<usize>{
        &self.items
    }
    pub fn remove_item(&mut self, item_id: usize) -> Option<usize> {
        match self.items.iter().position(|x| *x == item_id) {
            Some(index) => Some(self.items.swap_remove(index)),
            None => None,
        }
    }
    pub fn add_item(&mut self, item_id: usize) {
        self.items.push(item_id)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn adds_item_to_the_items_in_player(){
        let mut player = Player::new(100, 100);
        player.add_item(1);
        assert_eq!(player.items[player.items.len() - 1], 1);
        player.add_item(2);
        assert_eq!(player.items[player.items.len() - 1], 2);
    }
    #[test]
    fn return_item_id_if_item_in_player_inventory(){
        let mut player = Player::new(100, 100);
        player.add_item(1);
        player.add_item(2);
        player.add_item(3);

        let expected_1 = Some(2);
        let expected_2 = Some(3);
        assert_eq!(player.remove_item(2), expected_1);
        assert_eq!(player.items.len(), 2);
        assert_eq!(player.remove_item(3), expected_2);
        assert_eq!(player.items.len(), 1);
    }
}
