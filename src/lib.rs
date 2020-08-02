fn partition(n : usize) -> Vec<usize> {
    // numbers from 1 to 9 : 9 bits
    // 2^9 possibilities = 512
    let mut result : Vec<usize> = Vec::new();
    for combination in 1..512 {
        if combination_sum(combination) == n {
            result.push(combination);
            println!("{}: {:#b}", n, combination);
        }
    }
    result
}

/// The sum of values corresponding to a particular combination
pub fn combination_sum(c: usize) -> usize {
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

/// All the possible combinations leading to a particular roll value
pub fn partitions() -> Vec<Vec<usize>> {
    let mut partitions : Vec<Vec<usize>> = Vec::new();
    partitions.resize(13, vec!());
    for i in 2..13 {
        partitions[i] = partition(i);
    }
    partitions
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
