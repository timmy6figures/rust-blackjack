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

    pub fn total_value(&self) -> Vec<u32> {
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

    pub fn to_string(&self) -> String {
        let mut out_string = String::from("");
        for card in &self.cards {
            out_string.push_str(&card.to_string()[..]);
            out_string.push_str("\n");
        }
        out_string
    }

}
