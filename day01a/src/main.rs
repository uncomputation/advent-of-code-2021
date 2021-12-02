fn main() {
    let count = include_str!("input.txt")
                    .lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>()
                    .windows(2)
                    .filter(|t| t[0] < t[1])
                    .count();
    println!("{}", count);
}
