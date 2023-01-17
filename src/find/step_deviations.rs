pub fn find_step_deviations(
    a: f32,
    mid_points: Vec<f32>,
    class_interval_diff_length: i32,
) -> Vec<f32> {
    let mut step_deviations = vec![];

    for mid_point in mid_points {
        step_deviations.push(((mid_point - a) / (class_interval_diff_length - 1) as f32));
    }

    step_deviations
}

#[test]
fn test_find_step_deviations() {
    let mid_points = vec![4., 8., 12., 16., 20., 24., 28., 32.];
    let a = 20 as f32;
    let class_interval_diff_length = 5;
    let step_deviations = find_step_deviations(a, mid_points, class_interval_diff_length);
    assert_eq!(step_deviations, vec![-4., -3., -2., -1., 0., 1., 2., 3.]);
}
