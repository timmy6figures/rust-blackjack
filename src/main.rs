#![allow(unused)]

use std::io::stdin;

mod card;
use crate::card::Card;

mod rank;
use crate::rank::Rank;

mod suit;
use crate::suit::Suit;

mod deck;
use crate::deck::Deck;

mod hand;
use crate::hand::Hand;

fn main() {

    let mut d = Deck::new();
    d.shuffle();

    let mut player_hand = Hand::new();
    let mut dealer_hand = Hand::new();

    let mut player_total: u32 = 500;

    'game: loop {

        player_hand.clear();
        dealer_hand.clear();

        let bet = get_bet_amount();

        player_hand.add_card(d.draw_card());
        dealer_hand.add_card(d.draw_card());
        player_hand.add_card(d.draw_card());
        dealer_hand.add_card(d.draw_card());


        println!("Your hand:");
        print!("{}", player_hand.to_string());
        println!("Value:");
        println!("{:?}", player_hand.total_value());


        // Check for blackjack
        if (player_hand.total_value().contains(&21)){
            println!("Blackjack!");
            println!("Dealer hand:");
            println!("{}", player_hand.to_string());
            println!("Value:");
            println!("{:?}", player_hand.total_value());
            if (dealer_hand.total_value().contains(&21)) {
                println!("Push!")
            } else {
                println!("You win!");
                player_total += bet * 2;
                break 'game; // This is kinda a cool feature
            }
        }


        let mut hitting: bool;
        let mut bust = true;

        loop {
            hitting = match get_next_move().as_str() {
                "H\n" => true, 
                "S\n"=> false,
                _ => panic!(),
            };

            if hitting {
                player_hand.add_card(d.draw_card());

                println!("Your hand:");
                print!("{}", player_hand.to_string());
                println!("Value:");
                println!("{:?}", player_hand.total_value());

                // Check if bust
                bust = true;
                for num in player_hand.total_value().into_iter(){
                    if num <= 21 {
                        bust = false;
                    }
                }

            } else {
                break;
            }


            if bust {break;} 
        } 


    }

}


fn get_bet_amount() -> u32 {

    // Get next move
    println!("How much would you like to bet?");
    let mut input_string = String::new();

    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    input_string.trim().parse::<u32>().unwrap()
}

fn get_next_move() -> String {

    // Get next move
    println!("Hit or stand?");
    let mut input_string = String::new();

    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    input_string
}
