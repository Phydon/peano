use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
struct Element<T> {
    value: T,
}

impl Element<u64> {
    fn successor(&self) -> u64 {
        self.value + 1
    }

    fn next_value(&self) -> Element<u64> {
        let next = Element{
            value: self.successor(),
        };

        next
    }
}

fn main() {
    let zero = Element{
        value: 0,
    };
    println!("{}", zero.value);

    for i in 0..=9 {
        let n = Element{
            value: zero.value + i,
        };

        let next_number = n.successor();
        println!("{next_number}");
    }

    let mut set: BTreeSet<u64> = BTreeSet::new();

    for i in 0..10 {
        let n = Element{
            value: i,
        };
        let result = n.next_value();
        add_to_set(&mut set, result);
    }

    println!("{:?}", set);
}

fn add_to_set(set: &mut BTreeSet<u64>, n: Element<u64>) {
    let num = n.value;
    set.insert(num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successor() {
        let a = Element{
            value: 123,
        };

        assert_eq!(a.successor(), 124);
    }

    #[test]
    fn test_next_value() {
        let a = Element{
            value: 234,
        };
        let b = Element{
            value: 235,
        };

        assert_eq!(a.next_value(), b);
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn test_panic() {
        panic!();
    }
}
