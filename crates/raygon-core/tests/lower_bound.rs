use raygon_core::*;

#[test]
fn test_lower_bound() {
    let b = [1, 3, 3, 5];
    assert_eq!(lower_bound(b.len(), |i| b[i] < 0), Some(0));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 1), Some(0));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 2), Some(1));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 3), Some(1));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 4), Some(3));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 5), Some(3));
    assert_eq!(lower_bound(b.len(), |i| b[i] < 6), None);
}
