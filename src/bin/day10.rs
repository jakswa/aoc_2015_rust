use aoc_2015_rust::input;

fn main() {
    let inp = input("10");
    println!("part1 answer: {}", see_say(&inp.trim(), 40));
    println!("part2 answer: {}", see_say(&inp.trim(), 50));
}

fn see_say(inp: &str, iters: usize) -> usize {
    let mut chars = inp.chars().collect::<Vec<char>>();
    (0..iters).for_each(|_i| {
        let mut new_chars: Vec<char> = Vec::new();
        let mut prev = chars[0];
        let mut cnt = 1;
        chars.iter().skip(1).for_each(|&c| {
            if c != prev {
                cnt.to_string().chars().for_each(|c| new_chars.push(c));
                new_chars.push(prev);
                prev = c;
                cnt = 1;
            } else {
                cnt += 1;
            }
        });
        cnt.to_string().chars().for_each(|c| new_chars.push(c));
        new_chars.push(prev);
        chars = new_chars;
    });
    chars.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_stuff() {
        assert_eq!(see_say(&"1", 5), 6);
    }
}
