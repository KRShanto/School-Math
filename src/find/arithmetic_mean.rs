pub fn find_arithmetic_mean(frequency_sum: i32, fixi_sum: f32) -> f32 {
    fixi_sum / frequency_sum as f32
}

#[test]
fn test_find_arithmetic_mean() {
    let frequency_sum = 100;
    let fixi_sum = 6190.0;
    let arithmetic_mean = find_arithmetic_mean(frequency_sum, fixi_sum);
    assert_eq!(arithmetic_mean, 61.9);
}
