use std::collections::BTreeSet;

struct NaturalNumber {
    value: u64,
    natural: bool,
}

fn main() {
    let mut set: BTreeSet<u64> = BTreeSet::new();

    let zero = NaturalNumber {
        value: 0,
        natural: true,
    };

    let zero_string = "0".to_string();

    add_to_set(&mut set, &zero_string);
    println!("{:?}", &set);
}

fn add_to_set(set: &mut BTreeSet<u64>, num: &String) {
    match num.parse::<u64>() {
        Ok(_) => set.insert(num.parse::<u64>().unwrap() as u64),
        Err(_) => false,
    };
}
