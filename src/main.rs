mod elevens_game;

fn main() {
    let debug = false;
    let mut wins = 0;

    for i in 0..100_000_000 {
        let mut deck = elevens_game::new_shuffled_deck(debug);
        if  elevens_game::run_game(&mut deck, debug) {
            wins += 1;
        };

        print!("\x1B[2J\x1B[1;1H");
        println!("Games:    {}", i);
        println!("Win Rate: {}", wins as f64/i as f64);
    }

    println!("Wins: {}", wins);
}
