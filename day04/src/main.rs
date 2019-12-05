const MIN: usize = 147981;
const MAX: usize = 691423;

fn is_valid(password: usize) -> bool {
    has_exactly_two_adjacent_digit(password) && is_non_decreasing(password)
}

fn has_exactly_two_adjacent_digit(mut n: usize) -> bool {
    let mut run_length = 1;
    let mut last_digit = 10;
    while n > 0 {
        let digit = n % 10;
        if digit == last_digit {
            run_length += 1;
        } else {
            if run_length == 2 {
                return true;
            }
            run_length = 1;
        }
        last_digit = digit;
        n /= 10;
    }
    run_length == 2
}

fn is_non_decreasing(mut n: usize) -> bool {
    let mut last_digit = 10;
    while n > 0 {
        let digit = n % 10;
        if last_digit < digit {
            return false;
        }
        last_digit = digit;
        n /= 10;
    }
    true
}

fn main() {
    println!("{}", has_exactly_two_adjacent_digit(11));
    println!("{}", has_exactly_two_adjacent_digit(111));
    println!("{}", has_exactly_two_adjacent_digit(1221));
    println!("{}", has_exactly_two_adjacent_digit(1122));
    println!("{}", has_exactly_two_adjacent_digit(1112222));

    println!(
        "{}",
        (MIN..=MAX).filter(|&password| is_valid(password)).count()
    )
}
