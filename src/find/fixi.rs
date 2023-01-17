pub fn find_fixi(frequency: Vec<i32>, mid_points: Vec<f32>) -> Vec<f32> {
    let mut fixi = vec![];
    for (count, mid_point) in frequency.iter().zip(mid_points) {
        fixi.push(*count as f32 * mid_point);
    }
    fixi
}

#[test]
fn test_find_fixi() {
    let frequency = vec![5, 10, 15, 20, 30, 16, 4];
    let mid_points = vec![29.5, 39.5, 49.5, 59.5, 69.5, 79.5, 89.5];

    let fixi = find_fixi(frequency, mid_points);

    assert_eq!(
        fixi,
        vec![147.5, 395.0, 742.5, 1190.0, 2085.0, 1272.0, 358.0]
    );
}
