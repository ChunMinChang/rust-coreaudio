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
#[test]
fn test_btye_array_to_string() {
    let c_str = CStr::from_bytes_with_nul(b"hello~!@#$%^&*()_-=+\0").unwrap();
    let v_u8 = c_str.to_bytes().to_vec();
    let v_i8 = v_u8.iter().map(|&e| e as i8).collect();
    let result = btye_array_to_string(v_i8).unwrap();
    let expected = c_str.to_str().unwrap().to_string();
    assert_eq!(expected, result);
}
