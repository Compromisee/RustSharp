pub struct ShopItem {
    pub name: String,
    pub description: String,
    pub cost: u32,
    pub is_gem: bool,
}

pub struct ShopState {
    pub items: Vec<ShopItem>,
}

impl ShopState {
    pub fn generate() -> Self {
        Self {
            items: vec![
                ShopItem { name: "Steady Hand".into(), description: "Cards cannot be forced to discard by CPU cheats.".into(), cost: 40, is_gem: false },
                ShopItem { name: "Coin Magnet".into(), description: "+3 coins per round won.".into(), cost: 35, is_gem: false },
                ShopItem { name: "Tip Jar".into(), description: "+2 coins per round regardless of outcome.".into(), cost: 25, is_gem: false },
            ]
        }
    }
}
