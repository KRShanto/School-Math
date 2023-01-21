pub fn find_class_interval_length(range: f32, class_interval_diff_length: i32) -> i32 {
    // range / class_interval_diff_length

    // if there is a remainder, then add 1 to the result
    let class_interval_length: f32 = range as f32 / class_interval_diff_length as f32;
    class_interval_length.ceil() as i32
}

#[test]
fn test_find_class_interval_length() {
    let range = 9.;
    let class_interval_diff_length = 3;
    let class_interval_length = find_class_interval_length(range, class_interval_diff_length);
    assert_eq!(class_interval_length, 3);
}
