use day9::{parse_ints, part2};

fn main() {
    let path = "./input.txt";
    parse_ints(path);
    let int = part2(path);
    println!("int: {}", int);
}
