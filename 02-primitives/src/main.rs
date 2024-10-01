use std::mem;

fn main() {
    let xs: [i32;5] = [1,2,3,4,5];

    println!("First element of the array: {}", xs[0]);
    println!("Length of the array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    for i in 0..xs.len() + 1{
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far",i),
        }
    }
}
