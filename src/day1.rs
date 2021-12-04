#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(entries: &[usize]) -> usize {
    entries
        .windows(2)
        .into_iter()
        .filter(|window| window[1] > window[0])
        .count()
}

#[aoc(day1, part2)]
fn part2(entries: &[usize]) -> usize {
    entries
        .windows(3)
        .into_iter()
        .map(|window| window.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2_test() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part2(&input), 5);
    }
}
