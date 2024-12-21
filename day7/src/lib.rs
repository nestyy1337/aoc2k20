use advent2k20::parse_input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn get() {
    let input = parse_input("./input.txt").unwrap();

    let mut bag_map: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    for line in input.iter() {
        let mut words = line.split_whitespace();

        let bag_words: Vec<&str> = words.by_ref().take(2).collect();
        let _: Vec<&str> = words.by_ref().take(1).collect();

        let bag = bag_words.join(" ");

        let requirement = words.collect::<Vec<&str>>().join(" ");
        let requirement = requirement
            .trim_start_matches("contain")
            .trim_end_matches('.')
            .trim();

        let mut requirements_vec = Vec::new();
        if !requirement.contains("no other bags") {
            for req in requirement.split(", ") {
                let mut parts = req.split_whitespace();
                let count: u32 = parts.next().unwrap().parse().unwrap();
                let req_bag: String = parts.take(2).collect::<Vec<&str>>().join(" ");
                requirements_vec.push((req_bag, count));
            }
        }
        bag_map.insert(bag, requirements_vec);
    }

    //part1
    let mut set = HashSet::new();
    for (bag, _) in &bag_map {
        if check_if_gold(bag, &bag_map) {
            println!("bag: {} can contain gold bag", bag);
            set.insert(bag);
        }
    }
    println!("bags: {}", set.len());

    //part2
    let total_bags = count_bags_inside_gold("shiny gold", &bag_map);
    println!("total_bags: {}", total_bags);
}

fn count_bags_inside_gold(bag: &str, map: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    if let Some(reqs) = map.get(bag) {
        reqs.iter()
            .map(|(req_bag, count)| count + count * count_bags_inside_gold(req_bag, map))
            .sum()
    } else {
        return 0;
    }
}

fn check_if_gold(bag: &String, map: &HashMap<String, Vec<(String, u32)>>) -> bool {
    if let Some(reqs) = map.get(bag) {
        if reqs
            .iter()
            .any(|(req_bag, _)| req_bag.contains("shiny gold"))
        {
            println!("bag: {} can hold gold", bag);
            return true;
        }

        for (req_bag, _) in reqs {
            if check_if_gold(&req_bag, map) {
                return true;
            }
        }
    }
    false
}
