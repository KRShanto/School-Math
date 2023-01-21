pub fn find_short_cut_mean(
    a: f32,
    fiui_sum: f32,
    frequency_sum: i32,
    class_interval_diff_length: i32,
) -> f32 {
    a + (fiui_sum / frequency_sum as f32) * (class_interval_diff_length - 1) as f32
}

#[test]
fn test_find_short_cut_mean() {
    let fiui_sum = -37.;
    let frequency_sum = 188;
    let class_interval_diff_length = 5;
    let a = 20_f32;
    let short_cut_mean =
        find_short_cut_mean(a, fiui_sum, frequency_sum, class_interval_diff_length);
    assert_eq!(short_cut_mean, 19.212767);
}
