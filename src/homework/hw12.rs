// hw12.rs

fn main() {
    // Приклад використання функцій
    let shipments = vec![8, 2, 2, 4, 4];
    println!("Кількість переміщень: {}", count_permutation(&shipments));

    // Приклад генерації відвантажень
    let generated = gen_shipments(5);
    println!("Згенеровані відвантаження: {:?}", generated);
}

pub fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();

    // Перевірка, чи можна розділити порівну
    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments.iter() {
        balance += load as i32 - avg as i32;
        moves += balance.abs();
    }

    moves
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    // Створимо вектор із повторенням значення 10
    vec![10; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equalizable() {
        let shipments = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&shipments), 4);
    }

    #[test]
    fn test_not_equalizable() {
        let shipments = vec![1, 2, 3];
        assert_eq!(count_permutation(&shipments), -1);
    }

    #[test]
    fn test_other_case() {
        let shipments = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&shipments), 7);
    }
}