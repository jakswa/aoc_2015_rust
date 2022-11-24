use aoc_2015_rust::input;
use std::collections::HashMap;
use std::collections::HashSet;

struct HappyStuff<'a> {
    names: HashSet<&'a str>,
    map: HashMap<(&'a str, &'a str), i64>,
}

fn main() {
    let inp = input("13");
    let happy_stuff = happy_map(&inp);
    println!("part1 answer: {}", dig(&Vec::new(), &happy_stuff));
    //println!("part2 answer: {}", part2(&inp.trim()));
}

fn happy_map(inp: &str) -> HappyStuff {
    let mut map = HashMap::new();
    let mut names: HashSet<&str> = HashSet::new();
    inp.lines().for_each(|line| {
        let lvec = line.split(" ").collect::<Vec<&str>>();
        let lvlen = lvec.len();
        let lvval = match lvec[2] {
            "gain" => lvec[3].parse::<i64>().unwrap(),
            "lose" => -lvec[3].parse::<i64>().unwrap(),
            _ => 0,
        };
        let end_name = lvec[lvlen - 1].trim_matches('.');
        map.insert((lvec[0], end_name), lvval);
        names.insert(lvec[0]);
    });
    HappyStuff {
        map: map,
        names: names,
    }
}

fn happiness(seats: &Vec<&str>, happy: &HappyStuff) -> i64 {
    let mut sum = 0;
    seats[..].windows(2).for_each(|ab| {
        sum += happy.map.get(&(ab[0], ab[1])).unwrap();
        sum += happy.map.get(&(ab[1], ab[0])).unwrap();
    });

    let ablen = seats.len();
    sum += happy.map.get(&(seats[0], seats[ablen - 1])).unwrap();
    sum += happy.map.get(&(seats[ablen - 1], seats[0])).unwrap();
    sum
}

fn dig(seats: &Vec<&str>, happy: &HappyStuff) -> i64 {
    if seats.len() == happy.names.len() {
        return happiness(seats, happy);
    }
    let mut max = 0;
    happy.names.iter().for_each(|name| {
        if seats.contains(name) { return; }
        let mut new_seats = seats.clone();
        new_seats.push(name);
        let res = dig(&new_seats, happy);
        if res > max { max = res; }
    });
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_stuff() {
        assert_eq!(0, 0);
    }
}
