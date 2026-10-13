pub fn mirror_double(numbers: &[i32]) -> Vec<i32> {
    let result: Vec<i32> = numbers.to_vec();
    let result = result.iter().rev().map(|x| x * 2).collect();
    result
}
