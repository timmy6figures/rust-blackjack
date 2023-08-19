use std::fmt;

use Suit::*;

pub enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts
}

impl fmt::Display for Suit {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
