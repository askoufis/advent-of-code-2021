enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Command> {
    let lines = input.lines();
    lines
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            let name = split[0];

            let distance = split[1].parse().unwrap();
            match name {
                "forward" => Command::Forward(distance),
                "up" => Command::Up(distance),
                "down" => Command::Down(distance),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(entries: &[Command]) -> i32 {
    let mut depth = 0;
    let mut position = 0;

    entries.iter().for_each(|command| match command {
        Command::Forward(distance) => position += *distance as i32,
        Command::Up(distance) => depth -= *distance as i32,
        Command::Down(distance) => depth += *distance as i32,
    });

    depth * position
}

#[aoc(day2, part2)]
fn part2(entries: &[Command]) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim: i32 = 0;

    entries.iter().for_each(|command| match command {
        Command::Forward(distance) => {
            position += *distance as i32;
            depth += aim * *distance as i32;
        }
        Command::Up(distance) => aim -= *distance as i32,
        Command::Down(distance) => aim += *distance as i32,
    });

    depth * position
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn part1_test() {
        assert_eq!(part1(&INPUT), 150)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&INPUT), 900)
    }
}
