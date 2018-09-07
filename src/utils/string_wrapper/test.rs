use super::*;

// Tests for Public Functions
// ============================================================================

// to_string
// ------------------------------------
// Skip now ...

// Tests for Private Functions
// ============================================================================

// get_btye_array
// ------------------------------------
// Skip now ...

// btye_array_to_string
// ------------------------------------
// In fact, it's unnecessary to test it because it directly uses the built-in API.
#[test]
fn test_btye_array_to_string_with_invalid_vec() {
    // some invalid bytes, in a vector
    let bytes = vec![0, 159];
    assert!(btye_array_to_string(bytes).is_err())
}

#[test]
fn test_btye_array_to_string() {
    let expected = String::from("hello~!@#$%^&*()_-=+\0");
    let bytes = expected.clone().into_bytes();
    let result = btye_array_to_string(bytes).unwrap();
    assert_eq!(expected, result);
}
