fn main() {
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else if number == 5{
        println!("Equal");
    } else {
        println!("condition was falth");
    }

    if number != 0{
        println!("number was something other than zero")
    }

    let condition = true;
    let number = if condition{
        5
    }else{
        6
    };

    println!("The value of number is {}",number);

    //loop{
    //    println!("again!");
    //}

    let mut number = 3;

    while number != 0{
        println!("{}",number);
        number = number - 1;
    }
    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5{
        println!("the value is: {}",a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}",element);
    }

    for  number in (1..4).rev() {
        println!("{}!",number);
    }
    println!("LIFTOFF!!!")
}
