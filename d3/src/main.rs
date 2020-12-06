fn main() {
    p1();
    p2();
}

fn trees(input: &str, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut trees = 0;
    for (_, line) in input.lines().enumerate().filter(|(i, _)| i % down == 0) {
        if line.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x += right;
        if x >= line.len() {
            x -= line.len();
        }
    }
    trees
}

fn p1() {
    let input = include_str!("input");
    println!("{}", trees(input, 3, 1));
}

fn p2() {
    let input = include_str!("input");
    let first = trees(input, 1, 1);
    let second = trees(input, 3, 1);
    let third = trees(input, 5, 1);
    let fourth = trees(input, 7, 1);
    let fifth = trees(input, 1, 2);
    println!("{}", first * second * third * fourth * fifth)
}
