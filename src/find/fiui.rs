pub fn find_fiui(frequency: Vec<i32>, step_deviations: Vec<f32>) -> Vec<f32> {
    let mut fiui = vec![];
    for (count, step_deviation) in frequency.iter().zip(step_deviations) {
        fiui.push(*count as f32 * step_deviation);
    }
    fiui
}

#[test]
fn test_find_fiui() {
    let frequency = vec![1, 9, 21, 47, 52, 36, 19, 3];
    let step_deviations = vec![-4., -3., -2., -1., 0., 1., 2., 3.];
    let fiui = find_fiui(frequency, step_deviations);
    assert_eq!(fiui, vec![-4., -27., -42., -47., 0., 36., 38., 9.]);
}
