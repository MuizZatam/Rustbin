// This code panics if input is greater than the length of array 'a' i.e. greater than 4

use std::io;

fn main() {
    let a: [u8; 5] = [10; 5];

    println!("Enter an array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please input a number: ");

    let element = a[index];

    println!("Element at index {index} is {element}");
}
