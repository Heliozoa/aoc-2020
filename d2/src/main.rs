fn main() {
    p1();
    p2();
}

fn p1() {
    let input = include_str!("input");
    let c = input.lines().map(|l| {
        let mut split = l.split_whitespace();
        let limit = split.next().unwrap();
        let letter = split.next().unwrap();
        let password = split.next().unwrap();
        let mut limit = limit.split('-');
        let lower = limit.next().unwrap();
        let lower = usize::from_str_radix(lower, 10).unwrap();
        let upper = limit.next().unwrap();
        let upper = usize::from_str_radix(upper, 10).unwrap();
        let letter = letter.chars().next().unwrap();
        (lower, upper, letter, password)
    });

    let mut valid = 0;
    for (lower, upper, letter, password) in c {
        let count = password.chars().filter(|c| *c == letter).count();
        if count >= lower && count <= upper {
            valid += 1;
        }
    }
    println!("{}", valid);
}

fn p2() {
    let input = include_str!("input");
    let c = input.lines().map(|l| {
        let mut split = l.split_whitespace();
        let count = split.next().unwrap();
        let letter = split.next().unwrap();
        let password = split.next().unwrap();
        let mut count = count.split('-');
        let first = count.next().unwrap();
        let first = usize::from_str_radix(first, 10).unwrap();
        let second = count.next().unwrap();
        let second = usize::from_str_radix(second, 10).unwrap();
        let letter = letter.chars().next().unwrap();
        (first, second, letter, password)
    });

    let mut valid = 0;
    for (first, second, letter, password) in c {
        let first = password.chars().nth(first - 1).unwrap();
        let second = password.chars().nth(second - 1).unwrap();
        if first == letter {
            if second != letter {
                valid += 1;
            }
        } else if second == letter {
            valid += 1;
        }
    }
    println!("{}", valid);
}
