fn main() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("got : {val}");
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x * 2).collect();

    println!("v2 is : {:?}", v2);
}
