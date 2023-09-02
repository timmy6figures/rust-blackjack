use self::Rank::*;
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Rank {

    // This is a hash set because it returns a list of potential values; Ace can be 1 or 11
    pub fn value(&self) -> Vec<u32> {
        match self {
            Ace => Vec::from([1,11]),
            Two => Vec::from([2]),
            Three => Vec::from([3]),
            Four => Vec::from([4]),
            Five => Vec::from([5]),
            Six => Vec::from([6]),
            Seven => Vec::from([7]),
            Eight => Vec::from([8]),
            Nine => Vec::from([9]),
            Ten => Vec::from([10]),
            Jack => Vec::from([10]),
            Queen => Vec::from([10]),
            King => Vec::from([10]),
        }
    }

    pub fn to_char(&self) -> char{
        match self {
            Ace => 'A',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
        }

    }

    pub fn from_char(c: char) -> Rank {
        match c {
            'A' | '1' => Ace,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            _ => panic!(),
        }

    }

    pub fn to_string(&self) -> &str {
        match self {
            Ace => "Ace",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
        }

    }
}
