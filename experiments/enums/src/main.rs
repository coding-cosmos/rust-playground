mod control_flow;
mod matches;
mod message;
// use message::Message;
use matches::Coin;

use crate::matches::value_in_cents;

fn main() {
    // let m = Message::Write(String::from("hello"));
    // m.call()

    // let x = null;
    // let some_no = Some(5);
    // let some_char = Some('e');

    // let absent_no:Option<i32> = None;

    // let x = 5;
    // let y = Some(5);

    // let sum = x+y;
    // let coin = Coin::Dime;
    // println!("{}",value_in_cents(coin));

    let coin = Coin::Quarter(matches::UsState::Alaska);

    // let value = value_in_cents(coin);
    // println!("{}",value);

    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The max is configured to be {max}"),
    //     _ =>(),
    // }

    // // Value matching ignoring all others
    // if let Some(max) = config_max{
    //     println!("The max is configured to be {max}");
    // }

    let mut count = 0;
    // match coin{
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}");
    } else {
        count += 1;
    }
}
