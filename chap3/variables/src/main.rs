fn main() {
    //Variables and mutability
    let mut x = 5;
    println!{"The value of x is {}",x}
    x = 6;
    println!{"The value of x is {}",x}

    //Differences between variables and constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINT is {}",MAX_POINTS);
}
