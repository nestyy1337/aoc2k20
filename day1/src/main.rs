use std::fs;
fn main() {
    // not doing structured type shit

    let path = "./src/input.txt";
    let file = fs::read(path).unwrap();
    let text = String::from_utf8(file).unwrap();
    let mut nums = vec![];
    for line in text.lines() {
        if let Ok(num) = line.parse::<usize>() {
            nums.push(num);
        }
    }

    // find two
    for i in nums.iter() {
        for j in nums.iter() {
            if j + i == 2020 {
                eprintln!("{:?}", j * i);
            }
        }
    }

    // find three
    for i in nums.iter() {
        for j in nums.iter() {
            let sub = i + j;
            let looking = usize::checked_sub(2020, sub);
            match looking {
                Some(looking) => {
                    if nums.contains(&looking) {
                        eprintln!("{:?}", j * i * looking);
                        return;
                    }
                }
                None => {}
            }
        }
    }
}
