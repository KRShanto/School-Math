pub fn find_a(frequency: Vec<i32>, mid_points: Vec<f32>) -> f32 {
    // find the largest frequency
    // find the index of the largest frequency
    // that index is the index of the mid point
    // that mid point is the a

    let mut largest_frequency = 0;
    let mut largest_frequency_index = 0;

    for (index, count) in frequency.iter().enumerate() {
        if *count > largest_frequency {
            largest_frequency = *count;
            largest_frequency_index = index;
        }
    }

    mid_points[largest_frequency_index]
}

#[test]
fn test_find_a() {
    let mid_points = vec![4., 8., 12., 16., 20., 24., 28., 32.];
    let frequency = vec![1, 9, 21, 47, 52, 36, 19, 3];
    let a = find_a(frequency, mid_points);
    assert_eq!(a, 20_f32);
}
