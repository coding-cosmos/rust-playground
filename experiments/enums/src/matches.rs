#[derive(Debug)]
pub enum UsState{
    Albama,
    Alaska,
}

pub enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin:Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("The state is {:?}",state);
            25
        },
        _ => 4,
    }
}