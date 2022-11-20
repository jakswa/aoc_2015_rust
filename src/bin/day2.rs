use aoc_2015_rust::input;

fn main() {
    let inp = input("2");
    println!("part1 answer: {}", part1(&inp));
    println!("part2 answer: {}", part2(&inp));
}

fn part1(inp: &str) -> i32 {
    inp.lines().map(|l| {
        let mut dims = l.split("x").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        dims.sort();
        2 * dims[0] * dims[1] + 2 * dims[1] * dims[2] + 2 * dims[2] * dims[0] + dims[0] * dims[1]
    }).sum::<i32>()
}

fn part2(inp: &str) -> i32 {
    inp.lines().map(|l| {
        let mut dims = l.split("x").map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        dims.sort();
        dims[0] * 2 + dims[1] * 2 + dims[0]*dims[1]*dims[2]
    }).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part1(&"2x3x4"), 58);
        assert_eq!(part1(&"1x1x10"), 43);
    }
}
