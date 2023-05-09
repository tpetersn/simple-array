use std::io;

fn main() {
    let a: [i32; 13] = [1, 2, 3, 4, 5, 6, 6, 10, 15, 18, 21, 30, 1001];
    println!("Enter an array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered is not a number.");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}


