pub fn find_fixi_sum(fixi: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for count in fixi {
        sum += count;
    }
    sum
}

#[test]
fn test_find_fixi_sum() {
    let fixi = vec![147.5, 395.0, 742.5, 1190.0, 2085.0, 1272.0, 358.0];
    let fixi_sum = find_fixi_sum(fixi);
    assert_eq!(fixi_sum, 6190.0);
}
