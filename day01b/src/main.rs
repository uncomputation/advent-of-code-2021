const WINDOW_SIZE: usize = 3;

fn main() {
    let input = include_str!("input.txt")
                    .lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>();
    let mut count = 0;
    // Observation: Since the window sizes are 3 numbers long and we are comparing
    // their sums, we can reduce this to only checking the numbers that are different
    // that is: A + B + C < B + C + D iff A < D. We jump ahead in our iteration and 
    // subtract the length of the vector just once at the beginning to avoid invalid
    // access.
    for i in 0..(input.len() - WINDOW_SIZE) {
        if input[i] < input[i + WINDOW_SIZE] {
            count += 1;
        }
    }
    println!("{}", count);
}
