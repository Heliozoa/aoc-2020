fn main() {
    p1();
    p2();
}

fn seat_id(line: &str) -> usize {
    let mut lower = 0;
    let mut upper = 127;
    for c in line[..7].chars() {
        match c {
            'F' => {
                upper -= (upper - lower) / 2 + 1;
            }
            'B' => {
                lower += (upper - lower) / 2 + 1;
            }
            _ => unreachable!(),
        }
    }
    let row = lower;

    let mut lower = 0;
    let mut upper = 7;
    for c in line[7..].chars() {
        match c {
            'L' => {
                upper -= (upper - lower) / 2 + 1;
            }
            'R' => {
                lower += (upper - lower) / 2 + 1;
            }
            _ => unreachable!(),
        }
    }
    let column = lower;

    row * 8 + column
}

fn p1() {
    let input = include_str!("input");
    let mut max = 0;
    for line in input.lines() {
        max = max.max(seat_id(line));
    }
    println!("{}", max);
}

fn p2() {
    let input = include_str!("input");
    let mut seats = [false; 995];
    for line in input.lines() {
        let seat_id = seat_id(line);
        seats[seat_id] = true;
    }
    for (i, seat) in seats.iter().copied().enumerate().skip(1) {
        if !seat && seats[i - 1] && seats.get(i + 1).copied().unwrap_or_default() {
            println!("{}", i)
        }
    }
}
