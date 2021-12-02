fn main() {
    let input = include_str!("input.txt")
                    .lines()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<u32>>();
    let mut count = 0;
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}
