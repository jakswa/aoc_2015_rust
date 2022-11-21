use aoc_2015_rust::input;

fn main() {
    let inp = input("6");
    println!("part1 answer: {}", part1(&inp.trim()));
    println!("part2 answer: {}", part2(&inp.trim()));
}

struct XmasLightCmd {
    instr: Xinst,
    coords: Vec<usize>,
}
#[derive(Debug, PartialEq)]
enum Xinst {
    Toggle,
    TurnOn,
    TurnOff
}

fn parse_cmd(line: &str) -> XmasLightCmd {
    let instr = if line.starts_with("toggle") {
        Xinst::Toggle
    } else if line.starts_with("turn on") {
        Xinst::TurnOn
    } else { //line.starts_with("turn off") {
        Xinst::TurnOff
    };
    let coords = line.split(|c| { !char::is_numeric(c) })
        .filter(|s| s != &"")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    XmasLightCmd { instr, coords }
}

fn part1(inp: &str) -> usize {
    let mut arr = [[false;1000];1000];
    inp.lines().map(parse_cmd).for_each(|cmd| {
        (cmd.coords[0]..=cmd.coords[2]).for_each(|x| {
            (cmd.coords[1]..=cmd.coords[3]).for_each(|y| {
                match cmd.instr {
                    Xinst::Toggle => arr[x][y] = !arr[x][y],
                    Xinst::TurnOn => arr[x][y] = true,
                    Xinst::TurnOff => arr[x][y] = false
                }
            });
        });
    });
    arr.iter().map(|a| a.iter().filter(|i| {**i}).count()).sum()
}

fn part2(inp: &str) -> i32 {
    let mut arr: [[i32;1000];1000] = [[0;1000];1000];
    inp.lines().map(parse_cmd).for_each(|cmd| {
        (cmd.coords[0]..=cmd.coords[2]).for_each(|x| {
            (cmd.coords[1]..=cmd.coords[3]).for_each(|y| {
                match cmd.instr {
                    Xinst::Toggle => arr[x][y] += 2,
                    Xinst::TurnOn => arr[x][y] += 1,
                    Xinst::TurnOff => {
                        if arr[x][y] > 0 {
                            arr[x][y] -= 1
                        }
                    }
                }
            });
        });
    });
    arr.iter().map(|a| a.iter().sum::<i32>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_stuff() {
        assert_eq!(parse_cmd(&"toggle 0,0 through 999,0").coords, vec![0,0,999,0]);
        assert_eq!(parse_cmd(&"toggle 0,0 through 999,0").instr, Xinst::Toggle);
    }
}
