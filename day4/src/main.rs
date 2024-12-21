use day4::serde_type_shit::*;

fn main() {
    let path = "./input.txt";
    let input = std::fs::read_to_string(&path).unwrap();

    let passports_data = preprocess_input(&input);
    println!("{:?}", passports_data);

    let mut passports = vec![];
    for passport_str in passports_data {
        // xdd
        match serde_urlencoded::from_str::<Passport>(&passport_str) {
            Ok(passport) => passports.push(passport),
            Err(err) => eprintln!(
                "Failed to deserialize passport: {}\nError: {}",
                passport_str, err
            ),
        }
    }
    println!("{:?}", passports.len());
}
