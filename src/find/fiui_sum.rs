pub fn find_fiui_sum(fiui: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for count in fiui {
        sum += count;
    }
    sum
}

#[test]
fn test_find_fiui_sum() {
    let fiui = vec![-4., -27., -42., -47., 0., 36., 38., 9.];
    let fiui_sum = find_fiui_sum(fiui);
    assert_eq!(fiui_sum, -37.0);
}
