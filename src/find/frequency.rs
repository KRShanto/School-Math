pub fn find_frequency(only_numbers: Vec<i32>, class_interval: Vec<(i32, i32)>) -> Vec<i32> {
    let mut frequency = vec![];

    for (start, end) in class_interval {
        let mut count = 0;
        for number in only_numbers.iter() {
            if start <= *number && *number <= end {
                count += 1;
            }
        }
        frequency.push(count);
    }

    frequency
}

#[test]
fn test_find_frequency() {
    let only_numbers = vec![
        70, 40, 35, 60, 55, 58, 45, 60, 65, 80, 70, 46, 50, 60, 65, 70, 58, 60, 48, 70, 36, 85, 60,
        50, 46, 65, 55, 61, 72, 85, 90, 68, 65, 50, 40, 56, 60, 65, 46, 76,
    ];
    let class_interval = vec![
        (35, 39),
        (40, 44),
        (45, 49),
        (50, 54),
        (55, 59),
        (60, 64),
        (65, 69),
        (70, 74),
        (75, 79),
        (80, 84),
        (85, 89),
        (90, 94),
    ];
    let frequency = find_frequency(only_numbers, class_interval);
    assert_eq!(frequency, vec![2, 2, 5, 3, 5, 7, 6, 5, 1, 1, 2, 1]);
}
