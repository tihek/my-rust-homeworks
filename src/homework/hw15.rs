fn main() {
    let letters = ['m', 'u', 'x', 'a', 's', 'l', 'o', 'n'];
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];

    let mut count = 0;

    for perm in permute(&digits) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // Побудуємо числа
        let num1 = m * 1000 + u * 100 + x * 10 + a;
        let num2 = a; // однозначне множення на a
        let result = s * 1000 + l * 100 + o * 10 + n;

        if num1 * num2 == result {
            count += 1;
            println!("  {}{}{}{}", m, u, x, a);
            println!("x       {}", a);
            println!("-------");
            println!("  {}{}{}{}", s, l, o, n);
            println!();
        }
    }

    println!("Кількість рішень: {}", count);
}

// Функція генерації перестановок
fn permute(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut arr = arr.to_vec();
    permute_helper(&mut arr, 0, &mut results);
    results
}

fn permute_helper(arr: &mut [i32], start: usize, results: &mut Vec<Vec<i32>>) {
    if start == arr.len() {
        results.push(arr.to_vec());
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        permute_helper(arr, start + 1, results);
        arr.swap(start, i);
    }
}
