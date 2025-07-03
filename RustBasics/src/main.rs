mod lib;
use lib::greet;

fn main() {
    println!("Hello, This is Bhargav's first line in Rust !");
    greet("Bhargav");

    let x = 5;
    let y = 10;
    let sum = x + y;
    println!("The sum of {} and {} is {}", x, y, sum);

    let mut a = 15; // This line will cause an error because x is immutable
    println!("The value of a is: {}", a);
    a = add(30,20); // This line is fine because a is mutable

    println!("The value of a after change is: {}", a);

    
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// fn add(x: i32, y: i32, z: i32) -> i32 {
//    x + y + z
// }