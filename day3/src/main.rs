use day3::{parse_map, walk_map};

fn main() {
    let path = "./input.txt";

    let mut map = parse_map(path).unwrap();
    let case1 = walk_map(&mut map, 1, 1);
    let case2 = walk_map(&mut map, 3, 1);
    let case3 = walk_map(&mut map, 5, 1);
    let case4 = walk_map(&mut map, 7, 1);
    let case5 = walk_map(&mut map, 1, 2);

    let result = case1 as i64 * case2 as i64 * case3 as i64 * case4 as i64 * case5 as i64;
    println!("Result: {}", result);
}
