fn main() {
    // let mut counter = 0;

    // let result = loop{
    //     counter += 1;

    //     if counter == 10{
    //         break counter * 2;
    //     }
    // };

    // // println!("The result is {result}");
    // let mut count = 0;
    // 'couting_up: loop{
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop{
    //         println!("remaining = {remaining}");
    //         if remaining == 9{
    //             break;
    //         }
    //         if count == 2{
    //             break 'couting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;
    // while number != 0{
    //     println!("{number}");
    //     number -= 1;
    // }
    //     println!("LIFTOFF");

    // let a = [1,2,3,4,5];

    // for elm in a {
    //     println!("The value is {elm}");
    // }

    for n in (1..4).rev(){
        println!("The number is {n}");
    }

    println!("LIFTOFF!");
}
