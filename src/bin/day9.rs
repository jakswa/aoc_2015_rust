use aoc_2015_rust::input;
use std::collections::HashMap;

fn main() {
    let inp = input("9");
    println!("part1 answer: {}", part1(&inp.trim()));
    println!("part2 answer: {}", part2(&inp.trim()));
}

fn part1(inp: &str) -> usize {
    let mut dists: Vec<(&str, &str, usize)> = Vec::new();
    let mut map: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    inp.lines().for_each(|line| {
        let split = line.split(" ").collect::<Vec<&str>>();
        let dist = split[4].parse::<usize>().unwrap();
        dists.push((split[0], split[2], dist));
        let mapdot = map.entry(split[0]).or_insert(Vec::new());
        mapdot.push((split[2], dist));
        let mapdot = map.entry(split[2]).or_insert(Vec::new());
        mapdot.push((split[0], dist));
    });
    let cities = map.keys().map(|i| *i).collect::<Vec<&str>>();
    dig(Vec::new(), &map, 0)
}

fn dig(mut visits: Vec<&str>, map: &HashMap<&str, Vec<(&str, usize)>>, dist: usize) -> usize {
    let mut min = usize::MAX;
    if visits.len() == map.len() {
        return dist;
    }
    map.keys().for_each(|city| {
        if visits.contains(city) {
            return;
        }
        let curr_city = visits.last().unwrap_or(&"Invalid");
        let mut new_visits = visits.clone();
        new_visits.push(city);
        let dist_add = map
            .get(city)
            .unwrap()
            .iter()
            .find(|(mcity, _d)| mcity == curr_city)
            .unwrap_or(&("wee", 0))
            .1;
        let res = dig(new_visits, &map, dist + dist_add);
        if res < min {
            min = res;
        }
    });
    min
}

fn part2(inp: &str) -> usize {
    let mut dists: Vec<(&str, &str, usize)> = Vec::new();
    let mut map: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    inp.lines().for_each(|line| {
        let split = line.split(" ").collect::<Vec<&str>>();
        let dist = split[4].parse::<usize>().unwrap();
        dists.push((split[0], split[2], dist));
        let mapdot = map.entry(split[0]).or_insert(Vec::new());
        mapdot.push((split[2], dist));
        let mapdot = map.entry(split[2]).or_insert(Vec::new());
        mapdot.push((split[0], dist));
    });
    dig2(Vec::new(), &map, 0)
}

fn dig2(visits: Vec<&str>, map: &HashMap<&str, Vec<(&str, usize)>>, dist: usize) -> usize {
    let mut max = 0;
    if visits.len() == map.len() {
        return dist;
    }
    map.keys().filter(|i| !visits.contains(i)).for_each(|city| {
        let curr_city = visits.last().unwrap_or(&"Invalid");
        let mut new_visits = visits.clone();
        new_visits.push(city);
        let dist_add = map
            .get(city)
            .unwrap()
            .iter()
            .find(|(mcity, _d)| mcity == curr_city)
            .unwrap_or(&("Invalid", 0))
            .1;
        let res = dig2(new_visits, &map, dist + dist_add);
        if res > max {
            max = res;
        }
    });
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_stuff() {
        let map = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(part1(&map), 605);
        assert_eq!(part2(&map), 982);
    }
}
