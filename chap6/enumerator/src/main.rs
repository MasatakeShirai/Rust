#[derive(Debug)]
enum UsState{
    Alabama,
    //etc...
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

    let coin = Coin::Nickel;
    println!("The coin is {}.",value_in_cents(coin));

    let coin = Coin::Penny;
    println!("The coin is {}.",value_in_cents(coin));

    let coin = Coin::Quater(UsState::Alabama);
    println!("The coin is {}.",value_in_cents(coin));
   
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }else{
        println!("It is not three!")
    }
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


