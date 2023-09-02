use std::vec::Vec;
use std::{fmt, thread};
use std::string::String;

use rand::thread_rng;
use rand::seq::SliceRandom;


use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;


pub struct Deck {
    // 52 Cards
    cards: Vec<Card>,
    delt_cards: Vec<Card>
}

impl Deck {

    pub fn new() -> Deck {
        Deck {
            cards: Card::all_cards(),
            delt_cards: Vec::new(),
        }
    }

    pub fn draw_card(&mut self) -> Card {
        let card: Card = self.cards.remove(0);
        self.delt_cards.push(card.clone());
        card
    }

    pub fn cards_left(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng()); 
    }

    pub fn undelt_cards_string(&self) -> String {

        let mut out_string: String  = String::from("Cards: \n");

        for card in self.clone().cards.iter() {
            out_string.push_str(&card.to_string());
            out_string.push_str("\n");
        }

        out_string

    }

    pub fn delt_cards_string(&self) -> String {

        let mut out_string: String = String::from("Delt cards: \n");

        for card in self.clone().delt_cards.iter() {
            out_string.push_str(&card.to_string());
            out_string.push_str("\n");
        }
        out_string
    }

//    fn from_cards(cards: Vec<Card>) -> Deck {
//       Deck {
//           cards: cards,
//           delt_cards: Vec::with_capacity(cards.len())
//       }
//    }
}

