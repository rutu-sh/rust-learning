enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}


fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let mut coin = Coin::Penny; 
    println!("value in cents: {}", value_in_cents(&coin));

    coin = Coin::Nickel;
    println!("value in cents: {}", value_in_cents(&coin));

    coin = Coin::Dime; 
    println!("value in cents: {}", value_in_cents(&coin));

    coin = Coin::Quarter; 
    println!("value in cents: {}", value_in_cents(&coin));   

}