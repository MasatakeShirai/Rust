enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn main() {
    let coin = Coin::Dime;
    println!("The coin is {}.",value_in_cents(coin));

    let coin = Coin::Penny;
    println!("The coin is {}.",value_in_cents(coin));
}

fn value_in_cents(coin:Coin)->u32{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");    
            1
        },
        Coin::Nickel => 1,
        Coin::Dime => 1,
        Coin::Quater => 1,
    }
}


