use std::io::stdin;

mod card;
mod rank;
mod suit;
mod deck;
use crate::deck::Deck;

mod hand;
use crate::hand::Hand;

fn main() {

    let mut d = Deck::new();
    d.shuffle();

    let mut player_hand = Hand::new();
    let mut dealer_hand = Hand::new();

    let mut bank_total: u32 = 500;

    'game: loop {

        player_hand.clear();
        dealer_hand.clear();

        println!("Bank: {}", bank_total);
        let bet = get_bet_amount();
        bank_total -= bet;

        player_hand.add_card(d.draw_card());
        dealer_hand.add_card(d.draw_card());
        player_hand.add_card(d.draw_card());
        dealer_hand.add_card(d.draw_card());


        println!("YOU:");
        print_hand(&player_hand);
        println!("DEALER face up card:");
        println!("{}\n", dealer_hand.first_card_string());


        // Check for blackjack
        if player_hand.possible_values().contains(&21){
            println!("Blackjack!");

            println!("DEALER:");
            print_hand(&dealer_hand);

            if dealer_hand.possible_values().contains(&21) {
                println!("Push!");
                bank_total += bet;
            } else {
                println!("You win!");
                bank_total += bet * 2;
            }
            continue 'game; 
        }


        let mut hitting: bool;
        let mut bust;

        loop {
            hitting = match get_next_move().as_str() {
                "H\n" => true, 
                "S\n"=> false,
                _ => panic!(),
            };

            if hitting {
                player_hand.add_card(d.draw_card());

                println!("YOU:");
                print_hand(&player_hand);

                // Check if bust
                bust = true;
                for num in player_hand.possible_values().into_iter(){
                    if num <= 21 {
                        bust = false;
                    }
                }

            } else {
                break;
            }


            if bust {println!("Bust!"); continue 'game;} 
            if !hitting {break;}
        } 

        println!("Stand");

        while dealer_hand.is_legal_hand() {
            dealer_hand.add_card(d.draw_card());
            for value in dealer_hand.possible_values() {
                if value >= 17 {
                    break;
                }
            }
        }

        println!("DEALER:");
        print_hand(&dealer_hand);

        println!("YOU:");
        print_hand(&player_hand);

        // Check if dealer bust
        if !dealer_hand.is_legal_hand() {
            println!("DEALER bust!");
            bank_total += bet * 2
        }

        // Check if push
        if dealer_hand.best_hand_value() == player_hand.best_hand_value() {
            println!("Push!");
            bank_total += bet;
        }
        // Check if win 
        if dealer_hand.best_hand_value() < player_hand.best_hand_value() {
            println!("You win!");
            bank_total += bet*2;
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
    println!("Hit or stand? (H or S)");
    let mut input_string = String::new();

    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    println!("");
    input_string

}

fn print_hand(h: &Hand) {
    print!("{}", h.to_string());
    print!("Value:");
    println!("{:?}\n", h.possible_values());

}
