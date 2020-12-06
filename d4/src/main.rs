fn main() {
    p1();
    p2();
}

#[derive(Default)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
}

fn p1() {
    let input = include_str!("input");

    let mut passport = Passport::default();
    let mut valid = 0;
    for line in input.lines() {
        if line.is_empty() {
            if passport.byr
                && passport.iyr
                && passport.eyr
                && passport.hgt
                && passport.hcl
                && passport.ecl
                && passport.pid
            {
                valid += 1;
            }
            passport = Passport::default();
            continue;
        }

        let fields = line.split_whitespace();
        for field in fields {
            let mut parts = field.split(':');
            let field_name = parts.next().unwrap();
            match field_name {
                "byr" => passport.byr = true,
                "iyr" => passport.iyr = true,
                "eyr" => passport.eyr = true,
                "hgt" => passport.hgt = true,
                "hcl" => passport.hcl = true,
                "ecl" => passport.ecl = true,
                "pid" => passport.pid = true,
                "cid" => (),
                _ => unreachable!(),
            }
        }
    }
    println!("{}", valid);
}

fn p2() {
    let input = include_str!("input");

    let mut passport = Passport::default();
    let mut valid = 0;
    for line in input.lines() {
        if line.is_empty() {
            if passport.byr
                && passport.iyr
                && passport.eyr
                && passport.hgt
                && passport.hcl
                && passport.ecl
                && passport.pid
            {
                valid += 1;
            }
            passport = Passport::default();
            continue;
        }

        let fields = line.split_whitespace();
        for field in fields {
            let mut parts = field.split(':');
            let field_name = parts.next().unwrap();
            let field_value = parts.next().unwrap();
            match field_name {
                "byr" => {
                    if let Ok(byr) = usize::from_str_radix(field_value, 10) {
                        passport.byr = byr >= 1920 && byr <= 2002
                    }
                }
                "iyr" => {
                    if let Ok(iyr) = usize::from_str_radix(field_value, 10) {
                        passport.iyr = iyr >= 2010 && iyr <= 2020
                    }
                }
                "eyr" => {
                    if let Ok(eyr) = usize::from_str_radix(field_value, 10) {
                        passport.eyr = eyr >= 2020 && eyr <= 2030
                    }
                }
                "hgt" => {
                    if field_value.len() < 4 {
                        continue;
                    }
                    let (hgt, unit) = field_value.split_at(field_value.len() - 2);
                    if let Ok(hgt) = usize::from_str_radix(hgt, 10) {
                        match unit {
                            "cm" => passport.hgt = hgt >= 150 && hgt <= 193,
                            "in" => passport.hgt = hgt >= 59 && hgt <= 76,
                            _ => {}
                        }
                    }
                }
                "hcl" => {
                    if field_value.len() != 7 {
                        continue;
                    }
                    passport.hcl = field_value.starts_with('#')
                        && usize::from_str_radix(&field_value[1..], 16).is_ok();
                }
                "ecl" => {
                    passport.ecl = matches!(
                        field_value,
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                    )
                }
                "pid" => {
                    passport.pid =
                        field_value.len() == 9 && field_value.chars().all(|c| c.is_digit(10))
                }
                "cid" => (),
                _ => unreachable!(),
            }
        }
    }
    println!("{}", valid);
}
