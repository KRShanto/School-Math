pub fn find_classified_mode(
    class_interval: Vec<(i32, i32)>,
    frequency: Vec<i32>,
    class_interval_diff_length: i32,
) -> f32 {
    // Get the highest frequency index
    let highest_frequency_index = frequency
        .iter()
        .position(|&x| x == *frequency.iter().max().unwrap())
        .unwrap();

    // the lower limit of the class interval where the highest frequency occurs
    let l = class_interval[highest_frequency_index].0;

    // the subsraction of the highest frequency and its previous frequency
    let f1 = frequency[highest_frequency_index] - frequency[highest_frequency_index - 1];

    // the subsraction of the highest frequency and its next frequency
    let f2 = frequency[highest_frequency_index] - frequency[highest_frequency_index + 1];

    let h = class_interval_diff_length;

    l as f32 + (f1 as f32 / (f1 + f2) as f32) * h as f32
}

#[test]
fn test_find_classified_mode() {
    let class_interval = vec![
        (31, 40),
        (41, 50),
        (51, 60),
        (61, 70),
        (71, 80),
        (81, 90),
        (91, 100),
    ];
    let frequency = vec![4, 6, 8, 12, 9, 7, 4];
    let class_interval_diff_length = 10;

    let classified_mode =
        find_classified_mode(class_interval, frequency, class_interval_diff_length);

    assert_eq!(classified_mode, 66.71429);
}
