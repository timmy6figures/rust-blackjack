use crate::card::Card;
use std::collections::HashSet;

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {

    pub fn new() -> Hand {
        Hand {
            cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, c: Card) {
       self.cards.push(c); 
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn is_legal_hand(&self) -> bool {
        for value in self.possible_values() {
           if (value <= 21){
               return true;
           }
        }
        false
    }

    pub fn possible_values(&self) -> Vec<u32> {
        let mut total: Vec<u32> = Vec::from([0]);
        for card in self.cards.iter() {
            if card.value().len() == 1 { // Only one value (not ace)
                total = total.into_iter().map(|x: u32| x + card.value()[0]).collect();
            } else {
                let last = match total.pop() {
                    Some(i) => i,
                    None => 0,
                };
                total.push(last + 1);
                total.push(last + 11);
            }

        }
        // Once we are done, we have every combination of cards possible in a vec
        // Remove duplicaltes (Ace + 3 is the same as 3 + Ace), and return
        total.drain(..).collect()
    }

    pub fn best_hand_value(&self) -> u32 {
        let mut current_best_value = 0;
        for value in self.possible_values() {
           if (value > current_best_value && value <= 21){
                current_best_value = value;
           }
        }
        current_best_value
    }

    pub fn first_card_string(&self) -> String {
        self.cards.first().unwrap().to_string()
    }

    pub fn to_string(&self) -> String {
        let mut out_string = String::from("");
        for card in &self.cards {
            out_string.push_str(&card.to_string()[..]);
            out_string.push_str("\n");
        }
        out_string
    }

}
