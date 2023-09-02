use crate::suit::Suit;
use crate::rank::Rank;
use std::fmt;


#[derive(Copy, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {

    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {suit, rank}
    }


    pub fn to_string(&self) -> String {
        format!("{} of {} ", self.rank.to_string(), self.suit.to_string())
    }

    pub fn all_cards() -> Vec<Card> {

        let mut cards = Vec::new();
        cards.push(Card {suit: Suit::Spades, rank: Rank::Ace});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Two});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Three});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Four});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Five});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Six});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Seven});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Eight});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Nine});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Ten});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Jack});
        cards.push(Card {suit: Suit::Spades, rank: Rank::Queen});
        cards.push(Card {suit: Suit::Spades, rank: Rank::King});
        
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Ace});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Two});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Three});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Four});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Five});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Six});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Seven});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Eight});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Nine});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Ten});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Jack});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::Queen});
        cards.push(Card {suit: Suit::Diamonds, rank: Rank::King});

        cards.push(Card {suit: Suit::Clubs, rank: Rank::Ace});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Two});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Three});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Four});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Five});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Six});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Seven});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Eight});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Nine});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Ten});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Jack});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::Queen});
        cards.push(Card {suit: Suit::Clubs, rank: Rank::King});

        cards.push(Card {suit: Suit::Hearts, rank: Rank::Ace});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Two});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Three});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Four});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Five});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Six});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Seven});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Eight});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Nine});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Ten});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Jack});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::Queen});
        cards.push(Card {suit: Suit::Hearts, rank: Rank::King});



        cards

    }


}

impl fmt::Display for Card {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank.to_string(), self.suit.to_string(),)
    }

}
