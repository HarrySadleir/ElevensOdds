use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Card {
    Num(u16),
    Jack,
    Queen,
    King,
    Nil
} 

pub fn new_shuffled_deck(debug: bool) -> Vec<Card> {
    let mut result = Vec::new();
    for i in 1..11 {
        for _ in 0..4 {
            result.push(Card::Num(i));
        }
    }
    for _ in 0..4 {
        result.push(Card::Jack);
        result.push(Card::Queen);
        result.push(Card::King);
    }

    result.shuffle(&mut thread_rng());
    if debug {
        println!("{:?}", result);
    }
    result
}

pub fn run_game(deck: &mut Vec<Card>, debug: bool) -> bool {
    let mut board: [Card; 9] = [Card::Nil; 9];
    for spot in &mut board {
        *spot = deck.pop().expect("This should never be None");
    }

    loop {
        if debug {
            println!("Deck size: {}", deck.len());
        }
        match find_match(&board) {
            Ok(positions) => {
                if debug {
                    
                }
                for pos in positions {
                    match deck.pop() {
                        Some(card) => board[pos] = card,
                        None => {
                            if debug {
                                println!("We have an empty deck! {:?}", board);
                            }
                            return true // The deck is empty
                        }
                    }
                }
            },
            Err(_) => {
                if debug {
                    println!("A match could not be found : {:?}", board);
                }
                return false // A match could not be found :( 
            }
        };
    }
}

fn find_match(board: &[Card; 9]) -> Result<Vec<usize>, ()> {
    for i in 0..8 {
        let mut looking_for: Vec<Card> = match board[i] {
            Card::Num(value) => vec![Card::Num(11-value)],
            Card::Jack => vec![Card::Queen, Card::King],
            Card::Queen => vec![Card::Jack, Card::King],
            Card::King => vec![Card::Queen, Card::King],
            _ => {
                return Err(());
            }
        };

        
        let mut k: Option<usize> = None;
        for (j, possible_match) in board.iter().enumerate().take(9).skip(i) {
            if looking_for.contains(possible_match) { 
                looking_for.retain(|card| card != possible_match);
                if looking_for.is_empty() {
                    match k {
                        Some(num) => return Ok(vec![i, j, num]),
                        None => return Ok(vec![i, j])
                    }
                } else {
                    k = Some(j);
                }
            }
        }
    }

    Err(())
}