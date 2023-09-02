#![allow(unused)]

mod card;
use crate::card::Card;

mod rank;
use crate::rank::Rank;

mod suit;
use crate::suit::Suit;

mod deck;
use crate::deck::Deck;

fn main() {

    let c = Card::new(Suit::Clubs, Rank::Ace);

    let mut d = Deck::new();


}



