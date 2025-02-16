use day12::part1;
use day12::part2::part2;

fn main() {
    let input = "./input.txt";
    let res1 = part1(input);
    println!("{}", res1);

    // one time i decided to be explicit
    let res2 = part2(input);
    println!("{}", res2);

    //just do fold
    let (x, y, _, _) = include_bytes!("../input.txt").split(|b| b == &b'\n').fold(
        (0, 0, 10, -1),
        |(x, y, wx, wy), i| {
            if i.is_empty() {
                return (x, y, wx, wy);
            }
            let v: isize = atoi::atoi(&i[1..]).unwrap();
            match i[0] {
                b'N' => (x, y, wx, wy - v),
                b'S' => (x, y, wx, wy + v),
                b'E' => (x, y, wx + v, wy),
                b'W' => (x, y, wx - v, wy),
                b if b == b'L' && v == 90 || b == b'R' && v == 270 => (x, y, wy, -wx),
                b if (b == b'L' || b == b'R') && v == 180 => (x, y, -wx, -wy),
                b if b == b'L' && v == 270 || b == b'R' && v == 90 => (x, y, -wy, wx),
                b'F' => (x + wx * v, y + wy * v, wx, wy),
                _ => unreachable!(),
            }
        },
    );
    println!("{}", x.abs() + y.abs());
}
