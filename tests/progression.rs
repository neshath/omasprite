#[test]
fn creator_levels_are_ordered_and_named() {
    let levels = [1_u8, 2, 3, 5, 10, 20];
    assert!(levels.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(levels.first(), Some(&1));
    assert_eq!(levels.last(), Some(&20));
}
