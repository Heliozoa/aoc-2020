fn main() {
    p1();
    p2();
}

fn p1() {
    let input = include_str!("input");
    let mut set = [false; 26];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            let count: usize = set.iter().map(|b| if *b { 1 } else { 0 }).sum();
            sum += count;
            set = [false; 26];
        }
        for c in line.chars() {
            let index = c as usize - 'a' as usize;
            set[index] = true;
        }
    }
    println!("{}", sum);
}

fn p2() {
    let input = include_str!("input");
    let mut set = [0; 26];
    let mut group_size = 0;
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            let count: usize = set.iter().filter(|a| **a == group_size).count();
            sum += count;
            group_size = 0;
            set = [0; 26];
            continue;
        }
        group_size += 1;
        for c in line.chars() {
            let index = c as usize - 'a' as usize;
            set[index] += 1;
        }
    }
    println!("{}", sum);
}
