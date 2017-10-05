extern crate rand;

use std::io;
use rand::distributions::{IndependentSample, Range};

fn human_play() -> i32 {
    println!("Enter a choice:\n1. Rock(R) 2. Paper(P) 3.Scissors(S)");
    let mut pchoice = String::new();
    io::stdin()
        .read_line(&mut pchoice)
        .ok()
        .expect("Failed to read input!");
    let input: i32 = pchoice.trim().parse().expect("Wanted a number");
        input
}

fn comp_play() -> i32 {
    let mut rng = rand::thread_rng();
    let range = Range::new(1,3);
    let num = range.ind_sample(&mut rng);
    num
}

fn compare(hplay: i32, cplay: i32) -> String {
    let compare_game = (hplay, cplay);
    if compare_game == (1, 1) || compare_game == (2, 2) || compare_game == (3, 3) {
        return "It's a tie!".to_string();
    } else if compare_game == (1, 3) || compare_game == (2, 1) || compare_game == (3, 2) {
        return "You win!".to_string();
    } else {
        return "You lose!".to_string();
    }
}

fn game() -> String {
    let hplay = human_play();
    let cplay = comp_play();
    println!("Computer choice:{}",cplay);
    compare(hplay, cplay)
}

fn main() {
  println!("{}", game());
}

