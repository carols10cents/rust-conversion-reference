// This should fail to compile if any of these code examples are wrong.

fn main() {
    // Doing the string conversions all in main so that the lifetimes can be inferred.

    let s = "I am a &str.";

    // &str => String
    let st: String = s.to_string();

    // String => &str
    let str: &str = st.as_slice();
}

fn int_to_string(x: int) -> String {
    x.to_string()
}

fn string_to_int(x: String) -> int {
    x.parse().unwrap()
}
