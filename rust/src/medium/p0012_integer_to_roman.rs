pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        rec(num, String::new())
    }
}

fn rec(num: i32, roman: String) -> String {
    match num {
        n if n >= 1000 => format!("M{}", rec(n - 1000, roman)),
        n if n >= 900 => format!("CM{}", rec(n - 900, roman)),
        n if n >= 500 => format!("D{}", rec(n - 500, roman)),
        n if n >= 400 => format!("CD{}", rec(n - 400, roman)),
        n if n >= 100 => format!("C{}", rec(n - 100, roman)),
        n if n >= 90 => format!("XC{}", rec(n - 90, roman)),
        n if n >= 50 => format!("L{}", rec(n - 50, roman)),
        n if n >= 40 => format!("XL{}", rec(n - 40, roman)),
        n if n >= 10 => format!("X{}", rec(n - 10, roman)),
        n if n >= 9 => format!("IX{}", rec(n - 9, roman)),
        n if n >= 5 => format!("V{}", rec(n - 5, roman)),
        n if n >= 4 => format!("IV{}", rec(n - 4, roman)),
        n if n >= 1 => format!("I{}", rec(n - 1, roman)),
        _ => roman,
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }
}
