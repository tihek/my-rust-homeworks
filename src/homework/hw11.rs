use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }
    (min_indices.0, min_indices.1, min_sum)
}

pub fn print_result(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:>2}", i)).collect::<Vec<_>>().join(" "));
    println!("data:    {}", data.iter().map(|v| format!("{:>2}", v)).collect::<Vec<_>>().join(" "));

    let (i, j, sum) = min_adjacent_sum(data);
    let mut pointer_line = vec!["   "; data.len()];
    pointer_line[i] = "\\";
    pointer_line[j] = "/";

    println!("         {}", pointer_line.join(" "));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[i], data[j], sum, i, j);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![10, 20, 30, 40, 50];
        assert_eq!(min_adjacent_sum(&data), (0, 1, 30));
    }
}