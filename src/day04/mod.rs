const len: u32 = 8;

pub fn main() {
    calculate_passwords(245182, 790572);
}

fn calculate_passwords(min: u32, max: u32) {
    let mut valid_passwords = 0;
    for password in min..max+1 {
        if validate_password(password) {
            valid_passwords += 1
        }
    }
    println!("valid passwords: {}", valid_passwords);
}

fn validate_password(password_int: u32) -> bool {
    let mut last_last_digit = 10;
    let mut last_digit = 10;
    let mut matching: bool = false;
    // Working backwards
    for i in 1..len-1 {
        let digit = (password_int % 10u32.pow(i)) / 10u32.pow(i-1);
        if last_digit == digit && last_last_digit != digit {
            matching = true;
        }
        if matching && last_last_digit == digit {
            matching = false
        }
        // Rule 2. Digits must increment in size.
        if last_digit < digit {
            return false
        }
        last_last_digit = last_digit;
        last_digit = digit;
    }
    matching
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ones() {
        assert_eq!(validate_password(111111), false);
    }

    #[test]
    fn test_not_decreasing() {
        assert_eq!(validate_password(223450), false);
    }

    #[test]
    fn test_nodouble() {
        assert_eq!(validate_password(123789), false);
    }

    #[test]
    fn test_pairs() {
        assert_eq!(validate_password(112233), true);
    }

    #[test]
    fn test_notriple() {
        assert_eq!(validate_password(123444), false);
    }

    #[test]
    fn test_one_double() {
        assert_eq!(validate_password(111122), true);
    }
}