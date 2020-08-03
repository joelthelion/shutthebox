extern crate rand;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use shutthebox::*;

#[cfg(debug_assertions)]
const SIMS : usize = 1000;

#[cfg(not(debug_assertions))]
const SIMS : usize = 500_000;

trait Policy {
    /// Returns (n, action)
    fn choose(&mut self, state: usize, roll: usize) -> Option<(usize, usize)>;
    /// FIXME : implement discount?
    fn set(&mut self, q: Q, reward: f64);
    fn set_verbose(&mut self, verbose: bool);
}

#[derive(Clone, Copy, Debug)]
struct ValueCount {
    samples : usize,
    reward : f64
}

#[derive(Debug, Clone, Copy)]
struct Q {
    state: usize,
    roll: usize,
    action: usize
}

struct QPolicy {
    partitions : Vec<Vec<usize>>,
    table : Vec<ValueCount>,
    epsilon : f64,
    rng : SmallRng,
    verbose : bool,
}

const MAX_MOVES : usize = 40;
impl QPolicy {
    fn new() -> QPolicy {
        let mut policy = QPolicy {
            partitions : partitions(),
            table : Vec::new(),
            epsilon : 0.05,
            rng : SmallRng::seed_from_u64(123),
            verbose : true,
        };
        policy.table.resize(512*13*MAX_MOVES, ValueCount { samples: 0, reward: 0.});
        policy
    }
    fn get(&self, state: usize, roll: usize, action: usize) -> ValueCount {
        self.table[state*13*MAX_MOVES + roll*MAX_MOVES + action]
    }
}

impl Policy for QPolicy {
    fn choose(&mut self, state: usize, roll: usize) -> Option<(usize,usize)> {
        let mut max_reward = std::f64::NEG_INFINITY;
        let mut best : Option<(usize, usize)> = None;
        let epsilon_value : f64 = self.rng.gen();
        for (n, &mv) in self.partitions[roll].iter().enumerate() {
            if mv & !state != 0 { // if move is not legal
                continue
            }
            let q_value = self.get(state, roll, n);
            if self.verbose {
                println!("{:#011b} : {:?}", mv, q_value);
            }
            let reward = if epsilon_value < self.epsilon {
                self.rng.gen()
            } else {
                if q_value.samples == 0 { std::f64::INFINITY } else { q_value.reward / q_value.samples as f64 }
            };
            if reward > max_reward {
                max_reward = reward;
                best = Some((n, mv));
            }
        }
        best
    }
    fn set(&mut self, q: Q, reward: f64) {
        let val = &mut self.table[q.state*13*MAX_MOVES + q.roll*MAX_MOVES + q.action];
        val.reward += reward;
        val.samples += 1;
    }
    fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose
    }
}

struct RandomPolicy {
    partitions : Vec<Vec<usize>>,
    rng : SmallRng
}

impl RandomPolicy {
    fn new() -> RandomPolicy {
        RandomPolicy {
            partitions : partitions(),
            rng : SmallRng::seed_from_u64(123)
        }
    }
}

impl Policy for RandomPolicy {
    fn choose(&mut self, state: usize, roll: usize) -> Option<(usize, usize)> {
        let possible_moves = self.partitions[roll]
            .iter()
            .enumerate()
            .filter(|(_, &p)| p & !state == 0)
            .map(|(n,&p)| (n,p))
            .collect::<Vec<_>>();
        if possible_moves.is_empty() {
            None
        } else {
            possible_moves.choose(&mut self.rng).cloned()
        }
    }
    fn set(&mut self, _q : Q, _reward: f64) {}
    fn set_verbose(&mut self, _verbose: bool) {}
}

fn game<PolicyT: Policy, RngT: rand::Rng>( policy: &mut PolicyT, rng : &mut RngT, verbose: bool) -> usize {
    // All digits available
    let mut state = 511;
    let mut qs : Vec<Q> = Vec::new();
    loop {
        let roll = rng.gen_range(1,7) + rng.gen_range(1,7);
        if verbose {
            println!("{:#010b} / {}", state, roll);
        }
        let choice = policy.choose(state, roll);
        if let Some((n, mv)) = choice {
            let new = state & !mv;
            if verbose {
                println!("\t{:#010b} -> {:#010b}", mv, new);
            }
            qs.push(Q{state, roll, action:n});
            if new == 0 {
                break
            }
            state = new;
        } else {
            break
        }
    }
    let result = combination_sum(state);
    if verbose {
        println!("{:?}", qs);
    }
    for &q in qs.iter().rev() {
        // FIXME use discount
        policy.set(q, 45.-(result as f64));
    }
    if verbose {
        println!("Score: {}", result);
    }
    result
}

fn expected_reward<P: Policy>(policy : &mut P, sims: usize) -> f64 {
    let mut rng = SmallRng::seed_from_u64(123);
    let mut sum = 0.;
    let verbose = sims <= 10_000;
    policy.set_verbose(verbose);
    for _ in 0..sims {
        sum += game(policy, &mut rng, verbose) as f64;
    }
    sum / (sims as f64)
}

fn main() {
    let partitions = partitions();
    println!("{:#?}", partitions);
    let mut policy = RandomPolicy::new();
    println!("Expected return for random strat: {:.2}", expected_reward(&mut policy, 1000));
    let mut policy = QPolicy::new();
    println!("Expected return for q strat: {:.2}", expected_reward(&mut policy, SIMS));
    policy.epsilon = 0.;
    println!("Expected return for q strat (epsilon 0): {:.2}", expected_reward(&mut policy, 10_000));
}
