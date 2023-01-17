pub fn find_cumulative_frequency(frequency: Vec<i32>) -> Vec<i32> {
    let mut cumulative_frequency = vec![];
    let mut sum = 0;
    for count in frequency {
        sum += count;
        cumulative_frequency.push(sum);
    }
    cumulative_frequency
}

#[test]
fn test_find_cumulative_frequency() {
    let frequency = vec![2, 2, 5, 3, 5, 7, 6, 5, 1, 1, 2, 1];
    let cumulative_frequency = find_cumulative_frequency(frequency);
    assert_eq!(
        cumulative_frequency,
        vec![2, 4, 9, 12, 17, 24, 30, 35, 36, 37, 39, 40]
    );
}
