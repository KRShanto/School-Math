pub fn find_frequency_sum(frequency: Vec<i32>) -> i32 {
    let mut sum = 0;
    for count in frequency {
        sum += count;
    }
    sum
}

#[test]
fn test_find_frequency_sum() {
    let frequency = vec![5, 10, 15, 20, 30, 16, 4];
    let frequency_sum = find_frequency_sum(frequency);
    assert_eq!(frequency_sum, 100);
}
