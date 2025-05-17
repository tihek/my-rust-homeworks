// hw05.rs

pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ((24, 60), 12),
            ((4, 6), 2),
            ((2, 3), 1),
            ((204, 68), 4),
            ((204, 16), 4),
            ((40, 10), 10),
            ((200, 200), 200),
            ((800, 200), 200),
            ((1210, 803), 11),
            ((1230, 900), 30),
        ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }
}