pub fn find_mid_points(class_interval: Vec<(i32, i32)>) -> Vec<f32> {
    let mut mid_point = vec![];
    for (start, end) in class_interval {
        mid_point.push((start + end) as f32 / 2.0);
    }
    mid_point
}

#[test]
fn test_find_mid_points() {
    let class_interval = vec![(46, 50), (51, 55), (56, 60), (61, 65), (66, 70)];
    let mid_points = find_mid_points(class_interval);
    assert_eq!(mid_points, vec![48.0, 53.0, 58.0, 63.0, 68.0]);
}
