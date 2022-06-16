use std::collections::BTreeSet;

#[derive(Debug)]
struct NaturalNumber {
    value: u64,
}

impl NaturalNumber {
    fn successor(&self) -> u64 {
        self.value + 1
    }

    fn next_value(&self) -> NaturalNumber {
        let next = NaturalNumber{
            value: self.successor(),
        };

        return next;
    }
}

fn main() {
    let zero = NaturalNumber{
        value: 0,
    };
    println!("{}", zero.value);

    for i in 0..=9 {
        let n = NaturalNumber{
            value: zero.value + i,
        };

        let next_number = n.successor();
        println!("{next_number}");
    }

    for i in 0..9 {
        let n = NaturalNumber{
            value: i,
        };
        let result = n.next_value();
        dbg!(result);
    }
}
