use aoc_2015_rust::input;

fn main() {
    let inp = input("1");
    println!("part 1: {}", part1(&inp));
    println!("part 2: {}", part2(&inp));
}

fn part1(inp: &str) -> i32 {
    inp.chars().fold(0, |lvl, c| {
        match c {
            ')' => lvl - 1,
            '(' => lvl + 1,
            _ => lvl
        }
    })
}

fn part2(inp: &str) -> usize {
    let mut lvl: i32 = 0;
    inp.chars().enumerate().find(|(_ind, c)| {
        match c {
            ')' => lvl -= 1,
            '(' => lvl += 1,
            _ => {}
        }
        lvl == -1
    }).unwrap().0 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part1(&"()))"), -2);
        assert_eq!(part2(&"()))"), 3);
    }
}

