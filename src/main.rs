fn main() {
    let int_x = 3i;
    let int_to_string = int_x.to_string();
    let int_to_str = int_to_string.as_slice();

    println!("Hello, world!");
}

fn int_to_string(x: int) -> String {
    x.to_string()
}

fn str_to_int(x: &str) -> int {
    from_str::<int>(x).unwrap()
}
