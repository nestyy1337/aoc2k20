use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Ch(u8),
    Ref(Vec<usize>),
    RefOr(Vec<usize>, Vec<usize>),
}

pub fn part1() {
    let rule_str = include_str!(".././rules.txt");

    let mut rulesz: Vec<(usize, Rule)> = rule_str
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (id_str, rest) = l.split_once(": ").unwrap();
            let id = id_str.parse::<usize>().unwrap();
            if l.contains("|") {
                let (a, b) = rest.split_once(" | ").unwrap();
                let right = a
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();
                let left = b
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();
                (id, Rule::RefOr(right, left))
            } else if rest.starts_with('"') {
                let char = rest.chars().nth(1).unwrap() as u8;
                (id, Rule::Ch(char))
            } else {
                let a: Vec<usize> = rest
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();
                (id, Rule::Ref(a))
            }
        })
        .collect();

    // [Debug Print] Show how many rules were parsed and what they look like
    println!("Parsed {} rules:", rulesz.len());
    for (rid, rule) in &rulesz {
        println!(" - Rule {}: {:?}", rid, rule);
    }

    let messages: Vec<&str> = include_str!("./../input.txt").lines().collect();

    rulesz.sort_unstable_by_key(|rule| rule.0);
    let rules_sorted: Vec<Rule> = rulesz.into_iter().map(|r| r.1).collect();

    let res = messages
        .iter()
        .filter(|msg| {
            // [Debug Print] Show which message we're trying to match
            println!("\nChecking message: {:?}", msg);

            let bytes = msg.as_bytes();
            let outcome = matches(bytes, &rules_sorted, 0)
                .map(|consumed| consumed == bytes.len())
                .unwrap_or(false);

            // [Debug Print] Show whether the entire message matched
            println!("Message {:?} => matched entire message? {}", msg, outcome);
            outcome
        })
        .count();

    println!("r1: {}", res);
}

fn matches(msgs: &[u8], rules: &[Rule], pos: usize) -> Option<usize> {
    // [Debug Print] Show the current slice and which rule index
    println!(
        "  -> matches called with rule index = {}, slice = {:?}",
        pos,
        String::from_utf8_lossy(msgs)
    );

    if msgs.is_empty() {
        println!("     msgs is empty => returning None");
        return None;
    }

    let result = match &rules[pos] {
        Rule::Ch(c) => {
            // [Debug Print] Show which rule we're trying
            println!(
                "     Rule::Ch({:?}) => comparing to msgs[0]={:?}",
                *c as char, msgs[0] as char
            );
            if *c == msgs[0] { Some(1) } else { None }
        }
        Rule::Ref(right) => {
            // [Debug Print] Show which rule we're trying
            println!(
                "     Rule::Ref({:?}) => trying each subrule in sequence",
                right
            );
            right.iter().try_fold(0, |acc, &r| {
                // [Debug Print] Indicate sub-rule attempts
                println!("       -> matching subrule {}", r);
                matches(&msgs[acc..], rules, r).map(|n| {
                    let new_total = acc + n;
                    println!(
                        "          subrule {} matched, total consumed so far: {}",
                        r, new_total
                    );
                    new_total
                })
            })
        }
        Rule::RefOr(r, l) => {
            println!(
                "     Rule::RefOr({:?}, {:?}) => trying first, then second if needed",
                r, l
            );
            r.iter()
                .try_fold(0, |acc, &r| {
                    println!("       -> matching subrule (OR branch) {}", r);
                    matches(&msgs[acc..], rules, r).map(|n| acc + n)
                })
                .or_else(|| {
                    // [Debug Print] The first alternative failed, try the second
                    println!("       First OR branch failed. Trying second OR branch now.");
                    l.iter().try_fold(0, |acc, &r| {
                        println!("         -> matching subrule (OR branch2) {}", r);
                        matches(&msgs[acc..], rules, r).map(|n| acc + n)
                    })
                })
        }
    };

    // [Debug Print] Show the final result for this rule
    println!(
        "  <- matches returning for rule {}, slice {:?}: {:?}",
        pos,
        String::from_utf8_lossy(msgs),
        result
    );
    result
}
