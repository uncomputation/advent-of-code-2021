fn main() {
    // Observation: Since the window sizes are 3 numbers long and we are comparing
    // their sums, we can reduce this to only checking the numbers that are different
    // that is: A + B + C < B + C + D iff A < D. We jump ahead in our iteration and 
    // subtract the length of the vector just once at the beginning to avoid invalid
    // access.
    let count = include_str!("input.txt")
                    .lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>()
                    .windows(4)
                    .filter(|pairs| pairs[0] < pairs[3])
                    .count();
    println!("{}", count);
}
