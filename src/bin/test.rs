pub fn main() {
    let a = vec!["big", "beans", "are", "good", "yi", "bar"];

    let b = a[1..3].to_vec();

    println!("{:?}", b);
}
