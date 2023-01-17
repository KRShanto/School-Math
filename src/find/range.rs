pub fn find_range(smallest: i32, largest: i32) -> f32 {
    ((largest - smallest) + 1) as f32
}

#[test]
fn test_find_range() {
    let smallest = 35;
    let largest = 90;
    let range = find_range(smallest, largest);
    assert_eq!(range, 56.);
}
