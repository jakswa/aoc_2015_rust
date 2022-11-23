use aoc_2015_rust::input;

fn main() {
    let inp = input("11");
    let mut inp_chars = inp.trim().chars().collect::<Vec<char>>();
    let mut p1_answer = part1(&mut inp_chars);
    println!("part1 answer: {:?}", p1_answer);
    println!("part2 answer: {:?}", part1(&mut p1_answer));
}

fn valid(s: &[char]) -> bool {
    let res = s
        .windows(3)
        .any(|w| w[0] as u8 == w[1] as u8 - 1 && w[1] as u8 == w[2] as u8 - 1)
        && !s.iter().any(|&c| c == 'l' || c == 'o' || c == 'i');
    if (!res) {
        return false;
    };
    let pair_find = s.windows(2).find(|w| w[0] == w[1]);
    if pair_find.is_none() {
        return false;
    }
    let pair = pair_find.unwrap();
    s.windows(2).any(|w| w[0] == w[1] && pair != w)
}

fn incr(s: &mut [char]) -> &mut [char] {
    let mut ind = s.len() - 1;
    loop {
        let mut overflow = true;
        if s[ind] == 'z' {
            s[ind] = 'a'
        } else {
            overflow = false;
            s[ind] = (s[ind] as u8 + 1) as char;
        }
        if !overflow {
            break;
        }
        ind -= 1;
    }
    s
}

fn part1(s: &mut [char]) -> &mut [char] {
    incr(s);
    while !valid(s) {
        incr(s);
    }
    s
}

fn part2(inp: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_stuff() {
        assert_eq!(
            part1(&mut ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']),
            ['a', 'b', 'c', 'd', 'f', 'f', 'a', 'a']
        );

        assert_eq!(valid(&['a', 'b', 'c', 'c', 'e', 'e', 'j', 'k']), true);
        assert_eq!(valid(&['a', 'b', 'c', 'd', 'f', 'f', 'a', 'a']), true);

        assert_eq!(valid(&['a', 'b', 'b', 'c', 'e', 'g', 'j', 'k']), false);
        assert_eq!(valid(&['a', 'b', 'b', 'c', 'e', 'f', 'f', 'g']), false);
        assert_eq!(valid(&['h', 'i', 'j', 'k', 'l', 'm', 'm', 'n']), false);
    }
}
