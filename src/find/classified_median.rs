pub fn find_classified_median(
    frequency: Vec<i32>,
    frequency_sum: i32,
    cumulative_frequency: Vec<i32>,
    class_interval: Vec<(i32, i32)>,
    class_interval_diff_length: i32,
) -> f32 {
    let median = frequency_sum / 2;
    // find out in which cumulative_frequency (index) the median lies
    let mut meadian_class_index = 0;
    for (i, &cumulative_frequency) in cumulative_frequency.iter().enumerate() {
        if cumulative_frequency >= median {
            meadian_class_index = i;
            break;
        }
    }
    // the lower limit of the class interval where the median lies
    let l = class_interval[meadian_class_index].0;
    // the cumulative frequency before the meadian class
    let fc = cumulative_frequency[meadian_class_index - 1];
    // the frequency of the meadian class
    let fm = frequency[meadian_class_index];
    // the class interval difference
    let h = class_interval_diff_length;

    let classified_median = l as f32 + (median - fc) as f32 * h as f32 / fm as f32;

    classified_median as f32
}

#[test]
fn test_find_classified_median() {
    let class_interval = vec![(30, 35), (36, 41), (42, 47), (48, 53), (54, 59), (60, 65)];
    let frequency = vec![3, 10, 18, 25, 8, 6];
    let cumulative_frequency = vec![3, 13, 31, 56, 64, 70];
    let frequency_sum = 70;
    let class_interval_diff_length = 6;

    let classified_median = find_classified_median(
        frequency,
        frequency_sum,
        cumulative_frequency,
        class_interval,
        class_interval_diff_length,
    );

    assert_eq!(classified_median, 48.96);
}
