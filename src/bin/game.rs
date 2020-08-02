extern crate rand;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use shutthebox::*;

fn game<RngT: rand::Rng>( rng : &mut RngT, partitions : &Vec<Vec<usize>> ) -> usize {
    // All digits available
    let mut state = 511;
    loop {
        let roll = rng.gen_range(2,13);
        println!("{:#b} / {}", state, roll);
        let possible_moves = partitions[roll]
            .iter()
            .filter(|&p| p & !state == 0)
            .collect::<Vec<_>>();
        if possible_moves.is_empty() {
            break
        }
        let &choice = possible_moves.choose(rng).unwrap();
        let new = state & !choice;
        println!("\t{:#b} -> {:#b}", choice, new);
        if new == 0 {
            break;
        }
        state = new;
    }
    let result = combination_sum(state);
    println!("Score: {}", result);
    result
}

fn expected_reward() -> f64 {
    let partitions = partitions();
    let mut rng = SmallRng::seed_from_u64(123);
    const N:usize = 1000;
    let mut sum = 0.;
    for _ in 0..N {
        sum += game(&mut rng, &partitions) as f64;
    }
    sum / (N as f64)
}

fn main() {
    let partitions = partitions();
    println!("{:#?}", partitions);
    println!("Expected return for random strat: {:.2}", expected_reward());
}
