fn printable<T, U>(t: T, u: U)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    println!("first value is : {}", t);
    println!("second value is : {}", u);
}

fn main() {
    printable(42, "string");
}
