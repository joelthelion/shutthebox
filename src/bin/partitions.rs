fn partition(n : usize) {
    fn combination_sum(c: usize) -> usize {
        let lsb : usize = 1;
        let mut sum : usize = 0;
        let mut rem = c;
        for digit in 1..10 {
            if lsb&rem == 1 {
                sum += digit;
            }
            rem = rem >> 1;
        }
        sum
    }
    // numbers from 1 to 9 : 9 bits
    // 2^9 possibilities = 512
    for combination in 1..512 {
        if combination_sum(combination) == n {
            println!("{}: {:#b}", n, combination);
        }
    }
}

fn main() {
    for i in 2..13 {
        partition(i);
    }
    println!("{}", 3);
}
