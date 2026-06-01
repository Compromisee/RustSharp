use crate::core::cards::{Card, Deck};
use crate::core::zones::ZoneCard;
use crate::core::cpu::CPUProfile;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPhase {
    Draw,
    ZoneReveal,
    SelectCards,
    RevealAndScore,
    CheatCallWindow,
    RoundEnd,
    MatchOver,
}

pub struct MatchState {
    pub cpu: CPUProfile,
    pub deck: Deck,
    pub player_hand: Vec<Card>,
    pub cpu_hand: Vec<Card>,
    pub player_selected: Vec<Card>,
    pub cpu_selected: Vec<Card>,
    pub current_zone: Option<ZoneCard>,
    
    pub player_score: u32,
    pub cpu_score: u32,
    
    pub phase: MatchPhase,
    
    pub cpu_cheated_this_round: bool,
    pub cheat_called: bool,
    pub call_correct: Option<bool>,

    pub match_timer: f32, // Used for animations like shakes/tells
}

impl MatchState {
    pub fn new(cpu: CPUProfile) -> Self {
        Self {
            cpu,
            deck: Deck::new(),
            player_hand: Vec::new(),
            cpu_hand: Vec::new(),
            player_selected: Vec::new(),
            cpu_selected: Vec::new(),
            current_zone: None,
            player_score: 0,
            cpu_score: 0,
            phase: MatchPhase::Draw,
            cpu_cheated_this_round: false,
            cheat_called: false,
            call_correct: None,
            match_timer: 0.0,
        }
    }
}

pub struct RunState {
    pub coins: u32,
    pub gems: u32,
    pub credibility: u32,
    pub current_round: u32,
    pub match_index: usize,
    pub max_credibility: u32,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            coins: 0,
            gems: 0,
            credibility: 5,
            current_round: 1,
            match_index: 0,
            max_credibility: 5,
        }
    }
}

pub enum GamePhase {
    MainMenu,
    RunActive(MatchState),
    Shop,
    GameOver,
    Victory,
}

pub struct GameState {
    pub run: RunState,
    pub phase: GamePhase,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            run: RunState::new(),
            phase: GamePhase::MainMenu,
        }
    }
}
