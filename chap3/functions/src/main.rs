fn main() {
    println!("Hello, world!");
    another_function(5,10);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("x:{}, y:{}",x,y);
}

fn another_function(x:i32, y:i32){
    println!("another function");
    println!("The value is x:{}, y:{}",x,y);
}
