use std::io;

fn main() {
    let a=[1,2,3,4,5];
    println!("Please enter an index array!");
    let mut index=String::new();
    io::stdin().read_line(& mut index).expect("Failed to read line");
    let index:usize=index.trim().parse().expect("Index should be a number");
    let element=a[index];
    println!("The value of element at index {index} is {element}");
}
