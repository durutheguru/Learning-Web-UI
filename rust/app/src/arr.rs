use std::io;


pub fn find() {
    let a: [i32; 5] = [29, 54, 65, 2, 4];
    let mut index = String::new();

    println!("Enter the index of the array: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim()
        .parse()
        .expect("Index must be a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}

