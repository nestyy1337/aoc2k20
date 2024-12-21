use day2::*;

fn main() {
    let path = "./input.txt";
    let data = parse_str(path).unwrap();

    let mut valid_count = 0;

    for line in data.lines() {
        match analyze_line(&line) {
            Ok(false) => {
                eprintln!("corrupted {:?}", line);
            }
            Ok(true) => {
                valid_count += 1;
            }
            Err(_) => {}
        }
    }
    println!("Valid passwords: {}", valid_count);
}
