use aoc_2015_rust::input;

fn main() {
    let inp = input("4");
    println!("input is: {}", inp.trim());
    println!("part1 answer: {}", part1(&inp.trim()));
    println!("part2 answer: {}", part2(&inp.trim()));
}

fn part1(inp: &str) -> i32 {
    let mut i = 0;
    loop {
        let d = md5::compute(format!("{}{}", inp, i));
        if d[0..2] == [0,0] && d[2] <= 15 {
            break;
        }
        i += 1
    }
    i
}

fn part2(inp: &str) -> i32 {
    let mut i = 0;
    loop {
        let d = md5::compute(format!("{}{}", inp, i));
        if d[0..3] == [0,0,0] {
            break;
        }
        i += 1
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_hash() {
        assert_eq!(part1(&"abcdef"), 609043);
        assert_eq!(part1(&"pqrstuv"), 1048970);
        assert_eq!(part1(&"iwrupvqb"), 346386);
    }
}
