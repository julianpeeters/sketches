use sketches;

#[test]
fn test_public_api() {
    assert_eq!(sketches::Two::VALUES, [&sketches::Two::_1, &sketches::Two::_2]);
}