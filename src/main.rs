fn main() {
    println!("Hello, world!");

    println!("{:?}", test_function(2, 4));
}

fn test_function(x: i32, y: i32) -> i32 {
    return x + y;
}
