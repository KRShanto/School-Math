pub fn find_discrete_class_interval(class_interval: Vec<(i32, i32)>) -> Vec<(f32, f32)> {
    // class interval: [(46, 50), (51, 55)]
    // discrete class interval: [(45.5, 50.5), (50.5, 55.5)]
    let mut discrete_class_interval = vec![];

    for (start, end) in class_interval {
        discrete_class_interval.push((start as f32 - 0.5, end as f32 + 0.5));
    }

    discrete_class_interval
}

#[test]
fn test_find_discrete_class_interval() {
    let class_interval = vec![(46, 50), (51, 55), (56, 60), (61, 65), (66, 70)];
    let discrete_class_interval = find_discrete_class_interval(class_interval);
    assert_eq!(
        discrete_class_interval,
        vec![
            (45.5, 50.5),
            (50.5, 55.5),
            (55.5, 60.5),
            (60.5, 65.5),
            (65.5, 70.5)
        ]
    );
}
