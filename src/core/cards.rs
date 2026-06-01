use macroquad::rand::gen_range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Iron,   // Gray
    Ember,  // Red
    Bone,   // White
    Glass,  // Blue
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8, // 1 to 10
}

impl Card {
    pub fn new(suit: Suit, rank: u8) -> Self {
        Self { suit, rank }
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in [Suit::Iron, Suit::Ember, Suit::Bone, Suit::Glass] {
            for rank in 1..=10 {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let len = self.cards.len();
        for i in 0..len {
            let j = gen_range(0, len as u32) as usize;
            self.cards.swap(i, j);
        }
    }

    pub fn draw(&mut self, count: usize) -> Vec<Card> {
        let mut drawn = Vec::new();
        for _ in 0..count {
            if let Some(card) = self.cards.pop() {
                drawn.push(card);
            }
        }
        drawn
    }
}
