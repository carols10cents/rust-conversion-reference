fn main() {
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
