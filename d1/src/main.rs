fn main() {
    p1();
    p2();
}

fn p1() {
    let input = include_str!("input");
    let input: Vec<_> = input
        .lines()
        .map(|s| isize::from_str_radix(s, 10).unwrap())
        .collect();
    for (i, entry) in input.iter().enumerate() {
        for other_entry in input.iter().skip(i) {
            if entry + other_entry == 2020 {
                println!("{}", entry * other_entry);
            }
        }
    }
}

fn p2() {
    let input = include_str!("input");
    let input: Vec<_> = input
        .lines()
        .map(|s| isize::from_str_radix(s, 10).unwrap())
        .collect();
    for (i, entry) in input.iter().enumerate() {
        for (j, other_entry) in input.iter().enumerate().skip(i) {
            for third_entry in input.iter().skip(j) {
                if entry + other_entry + third_entry == 2020 {
                    println!("{}", entry * other_entry * third_entry);
                }
            }
        }
    }
}
