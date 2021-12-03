fn more_ones(nums: &[u32], pos: usize) -> bool {
    let mut count: i32 = 0;
    for &num in nums {
        let bit = (num >> pos) & 1;
        if bit == 0 {
            count -= 1;
        } else {
            count += 1;
        }
    }
    count > 0
}

fn main() {
    let nums: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let bit_length = 12;
    let mut gamma = 0;
    for i in 0..bit_length {
        if more_ones(&nums, i) {
            gamma |= 1 << i;
        }
    }
    let epsilon = !gamma & ((1 << bit_length) - 1);
    println!("gamma: {}, epsilon: {}, product: {}", gamma, epsilon, gamma * epsilon);
}
