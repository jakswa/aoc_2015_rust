use aoc_2015_rust::input;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let inp = input("8");
    println!("part1 answer: {}", part1(&inp));
    println!("part2 answer: {}", part2(&inp));
}

fn awklen(mut word: &str) -> usize {
    word = &word[1..word.len() - 1];
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
    }
    let escapes = word.matches(r"\\").count()
        + word.matches(r#"\""#).count()
        + (RE.find_iter(word).count() * 3);

    escapes + 2
}

fn awklen2(mut word: &str) -> usize {
    word = &word[1..word.len() - 1];
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
    }
    let escapes = word.matches(r#"""#).count() + word.matches(r#"\"#).count();

    escapes + 4
}

fn part1(inp: &str) -> usize {
    inp.lines().map(awklen).sum()
}

fn part2(inp: &str) -> usize {
    inp.lines().map(awklen2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_stuff() {
        let s = r#""aaa\"aaa""#;
        assert_eq!(awklen2(s), 6);

        let s = r#""\x27""#;
        assert_eq!(awklen2(s), 5);

        let s = r#""ltlkfbrrdvk\\""#;
        assert_eq!(awklen(s), 3);

        let s = r#""bvm\x28aa\\\\\"pywuhaniox\\z\\hbp\xd7mold""#;
        assert_eq!(awklen(s), 13);

        let s = r#""qludrkkvljljd\\xvdeum\x4e""#;
        assert_eq!(awklen(s), 6);

        let s = r#""cyxdpkh\\\"""#;
        assert_eq!(awklen(s), 4);

        let s = r##""bvm\x28aa\\\\\"pywuhaniox\\z\\hbp\xd7mold""##;
        assert_eq!(awklen(s), 13);

        let s = r#""""#;
        assert_eq!(awklen(s), 2);

        let s = r#""aaa\"aaa""#;
        assert_eq!(awklen(s), 3);

        let s = r#""\x27""#;
        assert_eq!(awklen(s), 5);
    }
}
