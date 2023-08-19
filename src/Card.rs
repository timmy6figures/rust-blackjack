struct Card {
    pub suit: Suit,
    pub rank: Rank
}

impl Card {
    fn new(suit: Suit::Suit, rank: Rank::Rank) -> Self{
        Self {suit, rank}
    }


    pub fn to_string(&self) -> String {
        format!("{} {} ", self.suit.to_str(), self.rank.to_str())
    }

}
