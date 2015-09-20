// This should fail to compile if any of these code examples are wrong.

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // Doing the string conversions all in main so that the lifetimes can be inferred.

    let s = "I am a &str.";

    // &str => String
    let st: String = s.to_string();

    // String => &str
    let str: &str = &st;
}

// i32 ---------------------------------------------------------------
#[allow(dead_code)]
fn i32_to_u32(x: i32) -> u32 {
    x as u32
}

#[test]
fn test_i32_to_u32_happy() {
    let x = 3i32;
    let y = 3u32;
    assert_eq!(y, i32_to_u32(x));
}

#[allow(dead_code)]
fn i32_to_string(x: i32) -> String {
    x.to_string()
}

#[test]
fn test_i32_to_string_happy() {
    let x = 3i32;
    let y = "3";
    assert_eq!(y, i32_to_string(x));
}

#[allow(dead_code)]
fn i32_to_f64(x: i32) -> f64 {
    x as f64
}

#[test]
fn test_i32_to_f64_happy() {
    let x = 3i32;
    let y = 3.0f64;
    assert_eq!(y, i32_to_f64(x));
}

// u32 ---------------------------------------------------------------

#[allow(dead_code)]
fn u32_to_i32(x: u32) -> i32 {
    x as i32
}

#[test]
fn test_u32_to_i32_happy() {
    let x = 3u32;
    let y = 3i32;
    assert_eq!(y, u32_to_i32(x));
}

#[allow(dead_code)]
fn u32_to_string(x: u32) -> String {
    x.to_string()
}

#[test]
fn test_u32_to_string_happy() {
    let x = 3u32;
    let y = "3";
    assert_eq!(y, u32_to_string(x));
}

#[allow(dead_code)]
fn u32_to_f64(x: u32) -> f64 {
    x as f64
}

#[test]
fn test_u32_to_f64_happy() {
    let x = 3u32;
    let y = 3.0f64;
    assert_eq!(y, u32_to_f64(x));
}

// String ---------------------------------------------------------------

#[allow(dead_code)]
fn string_to_i32(x: String) -> i32 {
    x.parse().unwrap()
}

#[test]
fn test_string_to_i32_happy() {
    let x = "3".to_string();
    let y = 3i32;
    assert_eq!(y, string_to_i32(x));
}

#[allow(dead_code)]
fn string_to_u32(x: String) -> u32 {
    x.parse().unwrap()
}

#[test]
fn test_string_to_u32_happy() {
    let x = "3".to_string();
    let y = 3u32;
    assert_eq!(y, string_to_u32(x));
}

#[allow(dead_code)]
fn string_to_f64(x: String) -> f64 {
    x.parse().unwrap()
}

#[test]
fn test_string_to_f64_happy() {
    let x = "3.14".to_string();
    let y = 3.14f64;
    assert_eq!(y, string_to_f64(x));
}

// f64 ---------------------------------------------------------------

#[allow(dead_code)]
fn f64_to_i32(x: f64) -> i32 {
    x as i32
}

#[test]
fn test_f64_to_i32_happy() {
    let x = 3.14f64;
    let y = 3i32;
    assert_eq!(y, f64_to_i32(x));
}

#[allow(dead_code)]
fn f64_to_u32(x: f64) -> u32 {
    x as u32
}

#[test]
fn test_f64_to_u32_happy() {
    let x = 3.14f64;
    let y = 3u32;
    assert_eq!(y, f64_to_u32(x));
}

#[allow(dead_code)]
fn f64_to_string(x: f64) -> String {
    x.to_string()
}

#[test]
fn test_f64_to_string_happy() {
    let x = 3.14f64;
    let y = "3.14";
    assert_eq!(y, f64_to_string(x));
}

// Option --------------------------------------------------------

#[test]
fn test_option_handling_with_default_some() {
    let x = Some(1).unwrap_or(0);
    assert_eq!(x, 1);
}

#[test]
fn test_parse_option_handling_with_default_none() {
    let x = None.unwrap_or(0);
    assert_eq!(x, 0);
}

#[test]
fn test_parse_option_handling_expect_some() {
    let x = Some(1).expect("Parsing int from string failed");
    assert_eq!(x, 1);
}

#[test]
#[should_panic(expected = "Parsing int from string failed")]
#[allow(unused_variables)]
fn test_parse_option_handling_expect_none() {
    let x: i32 = None.expect("Parsing int from string failed");
}

#[test]
fn test_parse_option_handling_unwrap_some() {
    let x = Some(1).unwrap();
    assert_eq!(x, 1);
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
#[allow(unused_variables)]
fn test_parse_option_handling_unwrap_none() {
    let x: i32 = None.unwrap();
}

// Vectors ---------------------------------------------------------

#[test]
fn test_vec_to_slice_happy() {
    let x = vec!(1u8, 2u8, 3u8);
    static Y: &'static [u8] = &[1, 2, 3];
    assert_eq!(Y, &x[..]);
}

#[test]
fn test_vec_to_boxedslice_happy() {
    let x = vec!(1u8, 2u8, 3u8);
    let y: Box<[u8]> = Box::new([1u8, 2, 3]);
    assert_eq!(y, x.into_boxed_slice());
}

#[test]
fn test_slice_to_vec_happy() {
    let x = &[1u8, 2, 3];
    let y = vec!(1u8, 2u8, 3u8);
    assert_eq!(y, x.to_vec());
}

#[test]
fn test_slice_to_boxedslice_happy() {
    let x = &[1u8, 2, 3];
    let y = Box::new([1u8, 2, 3]);
    assert_eq!(y, Box::new(*x));
}

#[test]
fn test_boxedslice_to_vec_happy() {
    let x: Box<[u8]> = Box::new([1u8, 2, 3]);
    let y = vec!(1u8, 2u8, 3u8);
    assert_eq!(y, x.into_vec());
}

#[test]
fn test_boxedslice_to_slice_happy() {
    let x: Box<[u8]> = Box::new([1u8, 2, 3]);
    let y = [1u8, 2, 3];
    assert_eq!(y, &*x);
}
