use std::io;

fn main() {
      let t = ([1; 2], [3; 4]);
  let (a, b) = t;
  println!("{}", a[0] + t.1[0]); 
    let a = [1,2,3,4,5];
    println!("Please enter an array index");
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");
    
    let index:usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
