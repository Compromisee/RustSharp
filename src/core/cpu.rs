#[derive(Debug, Clone)]
pub enum CheatType {
    CardSwap,
    ScoreNudge,
    ExtraDraw,
    SuitFake,
    ConditionFlip,
    None
}

#[derive(Debug, Clone)]
pub struct CPUProfile {
    pub id: u32,
    pub name: String,
    pub personality: String,
    pub cheat_type: CheatType,
    pub cheat_frequency: f32, // 0.0 to 1.0
    pub reward: u32,
}

impl CPUProfile {
    pub fn get_round_1() -> Vec<CPUProfile> {
        vec![
            CPUProfile {
                id: 1,
                name: "Benny the Beginner".into(),
                personality: "Nervous. Sloppy.".into(),
                cheat_type: CheatType::CardSwap,
                cheat_frequency: 0.10,
                reward: 40,
            },
            CPUProfile {
                id: 2,
                name: "Quick Vince".into(),
                personality: "Fast talking. Impatient.".into(),
                cheat_type: CheatType::ScoreNudge,
                cheat_frequency: 0.15,
                reward: 45,
            },
            CPUProfile {
                id: 3,
                name: "Mumbles".into(),
                personality: "Quiet. Avoids eye contact.".into(),
                cheat_type: CheatType::ExtraDraw,
                cheat_frequency: 0.12,
                reward: 50,
            },
        ]
    }
    
    pub fn get_boss_1() -> CPUProfile {
        CPUProfile {
            id: 101,
            name: "LUCKY (Boss)".into(),
            personality: "The Sloppy Hustler".into(),
            cheat_type: CheatType::CardSwap,
            cheat_frequency: 0.15,
            reward: 100,
        }
    }
}
