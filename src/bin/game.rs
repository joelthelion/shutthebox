extern crate rand;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

use shutthebox::*;

// FIXME : implement epsilon greedy + initial curiosity

trait Policy {
    /// Returns (n, action)
    fn choose(&mut self, state: usize, roll: usize) -> Option<(usize, usize)>;
    /// FIXME : implement discount?
    fn set(&mut self, q: Q, reward: f64);
}

#[derive(Default, Clone, Copy, Debug)]
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
    epsilon : f64
}

const MAX_MOVES : usize = 40;
impl QPolicy {
    fn new() -> QPolicy {
        let mut policy = QPolicy {
            partitions : partitions(),
            table : Vec::new(),
            epsilon : 0.05
        };
        policy.table.resize_with(512*13*MAX_MOVES, Default::default);
        policy
    }
    fn get(&self, state: usize, roll: usize, action: usize) -> ValueCount {
        self.table[state*13*MAX_MOVES + roll*MAX_MOVES + action]
    }
}

impl Policy for QPolicy {
    fn choose(&mut self, state: usize, roll: usize) -> Option<(usize,usize)> {
        let mut max_reward = f64::NEG_INFINITY;
        let mut best : Option<(usize, usize)> = None;
        for (n, &mv) in self.partitions[roll].iter().enumerate() {
            if mv & !state != 0 { // if move is not legal
                continue
            }
            let q_value = self.get(state, roll, n);
            println!("{:?}", q_value);
            let reward = if q_value.samples == 0 { 0. } else { q_value.reward / q_value.samples as f64 };
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
}

fn game<PolicyT: Policy, RngT: rand::Rng>( policy: &mut PolicyT, rng : &mut RngT) -> usize {
    // All digits available
    let mut state = 511;
    let mut qs : Vec<Q> = Vec::new();
    loop {
        let roll = rng.gen_range(2,13);
        println!("{:#b} / {}", state, roll);
        let choice = policy.choose(state, roll);
        if let Some((n, mv)) = choice {
            let new = state & !mv;
            println!("\t{:#b} -> {:#b}", mv, new);
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
    println!("{:?}", qs);
    for &q in qs.iter().rev() {
        // FIXME use discount
        policy.set(q, result as f64);
    }
    println!("Score: {}", result);
    result
}

fn expected_reward<P: Policy>(policy : &mut P) -> f64 {
    let mut rng = SmallRng::seed_from_u64(123);
    const N:usize = 10000;
    let mut sum = 0.;
    for _ in 0..N {
        sum += game(policy, &mut rng) as f64;
    }
    sum / (N as f64)
}

fn main() {
    let partitions = partitions();
    println!("{:#?}", partitions);
    let mut policy = RandomPolicy::new();
    println!("Expected return for random strat: {:.2}", expected_reward(&mut policy));
    let mut policy = QPolicy::new();
    println!("Expected return for q strat: {:.2}", expected_reward(&mut policy));
}
