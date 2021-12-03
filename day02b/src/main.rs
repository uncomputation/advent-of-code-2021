fn main() {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = include_str!("input.txt")
                    .lines()
                    .map(|line| line.split_once(" ").unwrap())
                    .map(|(cmd, x)| (cmd, x.parse().unwrap()))
                    .collect::<Vec<(&str, u32)>>();
    for (cmd, x) in lines {
        match cmd {
            "forward" => {
                pos += x;
                depth += aim * x;
            },
            "down" => aim += x,
            "up" => aim -= x,
            _ => (),
        };
    }
    println!("{} x {} = {}", pos, depth, pos * depth);
}
