extern crate rand;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use shutthebox::*;

#[derive(Default, Clone, Copy)]
struct ValueCount {
    samples : usize,
    reward : f64
}

struct QPolicy {
    table : Vec<ValueCount>,
    epsilon : f64
}

const MAX_MOVES : usize = 20;
impl QPolicy {
    fn new() -> QPolicy {
        let mut policy = QPolicy {
            table : Vec::new(),
            epsilon : 0.05
        };
        policy.table.resize_with(512*12*MAX_MOVES, Default::default);
        policy
    }
    fn get(&self, state: usize, roll: usize, action: usize) -> ValueCount {
        self.table[state*12*MAX_MOVES + roll*MAX_MOVES + action]
    }
    fn set(&mut self, state: usize, roll: usize, action: usize, reward: f64) {
        let val = &mut self.table[state*12*MAX_MOVES + roll*MAX_MOVES + action];
        val.reward += reward;
        val.samples += 1;
    }
}

struct RandomPolicy<'a> {
    partitions : &'a Vec<Vec<usize>>,
    rng : SmallRng
}

impl<'a> RandomPolicy<'a> {
    fn new(partitions : &'a Vec<Vec<usize>>) -> RandomPolicy {
        RandomPolicy {
            partitions,
            rng : SmallRng::seed_from_u64(123)
        }
    }
    fn choose(&mut self, state: usize, roll: usize) -> Option<usize> {
        let possible_moves = self.partitions[roll]
            .iter()
            .filter(|&p| p & !state == 0)
            .collect::<Vec<_>>();
        if possible_moves.is_empty() {
            None
        } else {
            let &&choice = possible_moves.choose(&mut self.rng).unwrap();
            Some(choice)
        }
    }
    fn set(&mut self, state: usize, roll: usize, action: usize, reward: f64) { }
}

fn game<RngT: rand::Rng>( policy: &mut RandomPolicy, rng : &mut RngT, partitions : &Vec<Vec<usize>> ) -> usize {
    // All digits available
    let mut state = 511;
    loop {
        let roll = rng.gen_range(2,13);
        println!("{:#b} / {}", state, roll);
        let choice = policy.choose(state, roll);
        if let Some(mv) = choice {
            let new = state & !mv;
            println!("\t{:#b} -> {:#b}", mv, new);
            if new == 0 {
                break
            }
            state = new;
        } else {
            break
        }
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
    let mut policy = RandomPolicy::new(&partitions);
    for _ in 0..N {
        sum += game(&mut policy, &mut rng, &partitions) as f64;
    }
    sum / (N as f64)
}

fn main() {
    let partitions = partitions();
    println!("{:#?}", partitions);
    println!("Expected return for random strat: {:.2}", expected_reward());
}
