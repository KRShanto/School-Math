pub fn find_class_interval(
    smallest: i32,
    largest: i32,
    class_interval_diff_length: i32,
) -> Vec<(i32, i32)> {
    let mut class_interval = vec![];
    let mut start = smallest;
    let mut end = smallest + class_interval_diff_length - 1;
    while end <= (largest + class_interval_diff_length - 1) {
        class_interval.push((start, end));
        start = end + 1;
        end = start + class_interval_diff_length - 1;
        // console_log!(start, " - ", end);
    }
    class_interval
}

#[test]
fn test_find_class_interval() {
    let smallest = 6;
    let largest = 14;
    let class_interval_length = 3;
    let class_interval = find_class_interval(smallest, largest, class_interval_length);
    assert_eq!(class_interval, vec![(6, 8), (9, 11), (12, 14),]);
}
