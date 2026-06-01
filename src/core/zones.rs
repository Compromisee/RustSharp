#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    Peak,       // Highest single card wins
    Balance,    // Closest to 15, over 15 = 0
    SuitZone,   // Most matching suits wins
    OddZone,    // Only odds count
    EvenZone,   // Only evens count
    VoidZone,   // Lowest total wins
    IronWall,   // Iron cards worth double
    EmberSurge, // Ember cards +2 each
    WildZone,   // All suits match
}

#[derive(Debug, Clone)]
pub struct ZoneCard {
    pub z_type: ZoneType,
    pub name: String,
    pub description: String,
}

impl ZoneCard {
    pub fn get_all() -> Vec<ZoneCard> {
        vec![
            ZoneCard { z_type: ZoneType::Peak, name: "Peak Zone".into(), description: "Highest single card wins.".into() },
            ZoneCard { z_type: ZoneType::Balance, name: "Balance Zone".into(), description: "Closest to 15. Over = 0.".into() },
            ZoneCard { z_type: ZoneType::SuitZone, name: "Suit Zone".into(), description: "Most matching suits wins.".into() },
            ZoneCard { z_type: ZoneType::OddZone, name: "Odd Zone".into(), description: "Only odd ranks count.".into() },
            ZoneCard { z_type: ZoneType::EvenZone, name: "Even Zone".into(), description: "Only even ranks count.".into() },
            ZoneCard { z_type: ZoneType::VoidZone, name: "Void Zone".into(), description: "Lowest total wins.".into() },
            ZoneCard { z_type: ZoneType::IronWall, name: "Iron Wall".into(), description: "Iron cards worth double.".into() },
            ZoneCard { z_type: ZoneType::EmberSurge, name: "Ember Surge".into(), description: "Ember cards add +2 each.".into() },
            ZoneCard { z_type: ZoneType::WildZone, name: "Wild Zone".into(), description: "All suits count as same.".into() },
        ]
    }
}
