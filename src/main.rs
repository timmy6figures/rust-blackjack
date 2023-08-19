#![allow(dead_code)] 
#![allow(unused)]

use std::iter::FilterMap;
use std::array;

mod Rank;
mod Suit;

struct Card {
    suit: Suit::Suit,
    rank: Rank::Rank
}

impl Card {
    fn new(suit: Suit::Suit, rank: Rank::Rank) -> Self{
        Self {suit, rank}
    }


    pub fn print(&self){
        println!("Suit: {}", self.rank);
        println!("Rank: {}", self.suit);
    }

}

struct Deck {
    // 52 Cards
    cards: Vec<Card>,
    drawn_cards: Vec<Card>
    }

//impl Deck {
//    pub fn new() -> Self {
//        for i in 1..13 {
//            println!("{}", i);
//
//        }
//
//        Self {}
//    }
//
//
//    fn draw_card(&self) -> Card {
//       Card {suit: Suit::Suit::Clubs, rank: Rank::Rank::King} 
//    }
//
//    fn cards_left(&self) -> u32 {
//        32
//    }
//}
//

fn main() {
    println!("Hello, world!");
    let curr_hand = (1,1);

    let c = Card::new(Suit::Suit::Diamonds, Rank::Rank::Ace);
    
    c.print();
}




//    fn value(&self) -> u32 {
//        if self.rank.is_numeric() {
//            self.rank.to_digit(10).unwrap()
//        }else{
//            match self.rank {
//                'J' => 10,
//                'Q' => 10,
//                'K' => 10,
//                'A' => 11,
//                _ => {panic!()},
//            }
//        }
//    }
