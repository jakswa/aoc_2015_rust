use aoc_2015_rust::input;
use std::collections::HashMap;

fn main() {
    let inp = input("3");
    println!("part1 answer: {}", part1(&inp));
    println!("part2 answer: {}", part2(&inp));
}

fn part1(inp: &str) -> i32 {
    let mut pos: (i32, i32) = (0,0);
    let mut hist: HashMap<(i32,i32), i32> = HashMap::new();
    hist.insert(pos, 1);
    inp.chars().for_each(|c| {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            _ => {}
        }
        let cnt = hist.entry(pos).or_insert(0);
        *cnt += 1;
    });
    hist.len() as i32
}

fn part2(inp: &str) -> i32 {
    let mut pos: (i32, i32) = (0,0);
    let mut robot_pos: (i32, i32) = (0,0);
    let mut hist: HashMap<(i32,i32), i32> = HashMap::new();
    let mut toggle = false;
    hist.insert(pos, 2);
    inp.chars().for_each(|c| {
        let p = match c {
            '>' => (1,0),
            '<' => (-1, 0),
            '^' => (0,-1),
            'v' => (0,1),
            _ => (0,0)
        };

        toggle = !toggle;
        match toggle {
            true => { pos.0 += p.0; pos.1 += p.1 },
            false => { robot_pos.0 += p.0; robot_pos.1 += p.1 }
        }
        let cnt = hist.entry(pos).or_insert(0);
        *cnt += 1;
        let rcnt = hist.entry(robot_pos).or_insert(0);
        *rcnt += 1;
    });
    hist.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part1(&"^v^v^v^v^v"), 2);
    }
}
