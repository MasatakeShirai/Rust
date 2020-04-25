fn main() {
    println!("Hello, world!");
    another_function(5,10);
}

fn another_function(x:i32, y:i32){
    println!("another function");
    println!("The value is x:{}, y:{}",x,y);
}
