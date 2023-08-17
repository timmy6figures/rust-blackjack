#![allow(dead_code)] 
#![allow(unused)]

use std::iter::FilterMap;
use std::array

struct Card {
    suit: char,
    rank: char
}

impl Card {
    fn new(suit: char, rank: char) -> Self{
        Self {suit, rank}
    }

    fn suit(&self) -> char {
        self.suit 
    }
    fn rank(&self) -> char {
        self.rank
    }
    fn value(&self) -> u32 {
        if self.rank.is_numeric() {
            self.rank.to_digit(10).unwrap()
        }else{
            match self.rank {
                'J' => 10,
                'Q' => 10,
                'K' => 10,
                'A' => 11,
                _ => {panic!()},
        }
    }
    }
    
    pub fn print(&self){
        println!("Suit: {}", self.suit());
        println!("Rank: {}", self.rank);
        println!("Value: {}", self.value());
    }
}

struct Deck {
    // 52 Cards
    cards: Vec<Card>,
    }

impl Deck {
    pub fn new() -> Self {
        let arr = [Card; 13];
            for i in 1..13 {
                println!("{}", i);
                
            }
        Self {}
    }


    fn draw_card(&self) -> Card {
        'J'
    }

    fn cards_left(&self) -> u32 {
        32
    }
}
//
//struct Player {
//
//}
//
//impl Player {
//    fn next_move(opp_card: i32){ }
//    fn hand_value(opp_card: i32){ }
//
//}

fn main() {
    println!("Hello, world!");
    let curr_hand = (1,1);

    let c = Card::new('D', '2');
    let p = Card::new('D', 'Q');
    
    c.print();
    p.print();
}
