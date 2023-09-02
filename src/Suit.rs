use self::Suit::*;

#[derive(Copy, Clone)]
pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

impl Suit {

    pub fn to_char(&self) -> char {
        match self {
            Spades => 'S',
            Clubs => 'C',
            Hearts => 'H',
            Diamonds => 'D',
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Spades => "Spades",
            Clubs => "Clubs",
            Diamonds => "Diamonds",
            Hearts => "Hearts",
        }
    }
}
