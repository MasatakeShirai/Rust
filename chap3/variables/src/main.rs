fn main() {
    //Variables and mutability
    let mut x = 5;
    println!{"The value of x is {}",x}
    x = 6;
    println!{"The value of x is {}",x}

    //Differences between variables and constants
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINT is {}",MAX_POINTS);

    //Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!{"The value of y is {}",y}

    //mut can't change type, but shadowing can change type
    let space = "     ";
    let space = space.len();
    println!("The lengrh of space is {}",space);

    //Numerical calculation
    let sum =  5 + 10;
    let diffence = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum:{}, diffence:{}, product:{}, quotient:{}, remainder:{}"
             ,sum ,diffence, product, quotient, remainder);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let(x,y,z) = tup;
    println!("x:{}, y:{}, z:{}",x,y,z);

    let five_hundred = tup.0;
    let six_point_fout = tup.1;
    let one = tup.2;
    println!("{}, {}, {}",five_hundred, six_point_fout, one);

    //array 
    let a = [0,1,2,3,4,5];
    let first = a[0];
    let second = a[1];
    println!("first:{}, second:{}",first,second);
}
