fn main() {
    let mut s1 = String::from("Hello");
    let s2 = s1.clone();
    s1.replace_range(0..1, "y");
    println!("{s1}, {s2}");
}
