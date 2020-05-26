#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    NY,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn main() {
    let coin = Coin::Dime;
    println!("The coin is {}.",value_in_cents(coin));

    let coin = Coin::Penny;
    println!("The coin is {}.",value_in_cents(coin));

    let coin = Coin::Quater(UsState::Alabama);
    println!("The coin is {}.",value_in_cents(coin));
   
}

fn value_in_cents(coin:Coin)->u32{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");    
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("The state is {:?}!",state);
            25
        },
    }
}


