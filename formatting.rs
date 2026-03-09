// fn main(){
    // println!("{} days",31);

    // println!("{0}, this is {1}. {1}, this is {0}","Alice","Bob");

    // println!("{name} study in {school}", name="Bob", school="Harvard");

    // Coverting the numbers
    // println!("Base 10:          {} ",787878);
    // println!("Base  2:          {:b}",787878);
    // println!("Base  8:          {:o}",787878);
    // println!("Base 16:          {:x}",787878);

    // println!("My name is {0}, {1} {0}", "Bond","James");

    // let number: f64 = 1.0;
    // let width : usize = 5;

    // println!("{number:>width$}");

    // let pi:f64 = 3.141592;
    // println!("Pi is roughly {pi:.3}");

    // println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
//     #[derive(Debug)]
//     struct Structure(i32);

//     #[derive(Debug)]
//     struct Deep(Structure);

//     fn main(){
//         println!("{:?} months in a year.",12);
//         println!("{1:?} {0:?} is the {actor:?} name.",
//                 "Slater",
//                 "Christian",
//                 actor="actor's");
//         println!("Now {:?} will print!",Structure(3));
//         println!("Now {:?} will print!", Deep(Structure(7)));
//     }
// // }

// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

// fn main(){
//     let name = "Peter";
//     let age = 27;
//     let peter = Person {name, age};

//     println!("{:#?}", peter);
// }
// fn main(){
//     use std::fmt;

//     struct Structure(i32);

//     impl fmt::Display for Structure{
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f,"{}",self.0)
//         }
//     }
// }


use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);


impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})",self.0,self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64
}
 
impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "x: {}, y: {}",self.x, self.y)
    }
}

fn main(){
    let minmax = MinMax(0,14);

    println!("Compare Structres");
    println!("Display: {}",minmax);
    println!("Debug: {:?}",minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}", small=small_range, big = big_range);

    let point = Point2D{x:3.3, y:7.2};

    println!("Compare points:");
    println!("Display: {}",point);
    println!("Debug: {:?}",point);

    
}










