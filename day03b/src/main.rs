fn count_at_pos(nums: &[u32], pos: usize) -> (u32, u32) {
    let mut counts = (0, 0);
    for &num in nums {
        let bit = (num >> pos) & 1;
        if bit == 0 {
            counts.0 += 1;
        } else {
            counts.1 += 1;
        }
    }
    counts
}

fn process_numbers(nums: &[u32], bit_length: usize, default_value: u32, secondary_value: u32) -> u32 {
    let mut nums = nums.to_vec();
    for i in (0..bit_length).rev() {
        let (zeros, ones) = count_at_pos(&nums, i);
        // >= because we default to 1 if the number of ones equals number of zeros
        let common = if ones >= zeros { default_value } else { secondary_value };
        nums.retain(|&x| (x >> i) & 1 == common);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

fn main() {
    let nums: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();
    let bit_length = 12;
    let o2_generator_rating = process_numbers(&nums, bit_length, 1, 0);
    let co2_scrubber_rating = process_numbers(&nums, bit_length, 0, 1);
    println!("oxygen generator rating: {}, CO2 scrubber rating: {}, life support rating: {}",
    o2_generator_rating, co2_scrubber_rating, o2_generator_rating * co2_scrubber_rating);
}
