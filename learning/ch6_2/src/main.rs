#[derive(Debug)]

enum UsState {
    Austin,
    Texas,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cent(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        } 
    }
}


fn main() {
    let quarter = Coin::Quarter(UsState::NewYork);
    let value_of_quarter = value_in_cent(quarter);
    println!("value of quarter is {}", value_of_quarter);

}