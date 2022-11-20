use aoc_2015_rust::input;

fn main() {
    let inp = input("5");
    println!("part1 answer: {}", part1(&inp.trim()));
    println!("part2 answer: {}", part2(&inp.trim()));
}

fn part1(inp: &str) -> usize {
    inp.lines().filter(|w| {
        let mut grps = w.chars().zip(w.chars().skip(1));
        w.chars().filter(|i| ['a', 'e', 'i', 'o','u'].contains(i)).count() > 2 &&
            grps.any(|(a,b)| a == b) &&
            !(w.contains("ab") || w.contains("cd") || w.contains("pq") || w.contains("xy"))
    }).count()
}

fn part2(inp: &str) -> usize {
    inp.lines().filter(|w| has_oddchar(w)).filter(|w| has_chunk_pair(w)).count()
}

fn has_chunk_pair(w: &str) -> bool {
    w.as_bytes().windows(2).enumerate().any(|(ind,i)| {
        w[ind+2..].as_bytes().windows(2).any(|j| j == i)
    })
}

fn has_oddchar(w: &str) -> bool {
    let mut grps = w.chars().zip(w.chars().skip(2));
    grps.any(|(a,b)| a == b)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oddchar_works() {
        assert_eq!(has_oddchar(&"ieodomkazucvgmuy"), true);
    }

    #[test]
    fn chunk_pair_detects() {
        assert_eq!(has_chunk_pair(&"awefghaweuogh"), true);
        assert_eq!(has_chunk_pair(&"awwwofwaef"), false);
    }

    #[test]
    fn finds_nice_words() {
        assert_eq!(part1(&"ugknbfddgicrmopn"),1);

        assert_eq!(part2(&"qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2(&"xxyxx"), 1);
        assert_eq!(part2(&"uurcxstgmygtbstg"), 0);
        assert_eq!(part2(&"ieodomkazucvgmuy"), 0);
    }
}
