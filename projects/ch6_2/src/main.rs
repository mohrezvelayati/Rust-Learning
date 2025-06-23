enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cent(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let quarter = Coin::Quarter;
    let value_of_quarters = value_in_cent(quarter);
    println!("value of quarter is {}", value_of_quarters);


    let penny = Coin::Penny;
    let value_of_penny = value_in_cent(penny);
    println!("value of penny is {}", value_of_penny);
}