fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}",s);

    //move(Shallow copy)
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}",s2);

    //clone(Deep copy)
    let s1 = String::from("Hay");
    let s2 = s1.clone();
    println!("{},{}",s1,s2);

    //copy
    let x = 7;
    let y = x;
    println!("x:{}, y:{}",x,y);

    //s gose into the scope
    let s = String::from("GoodNight..");
    //s moves to func
    takes_ownership(s);

    //x gose into the scope
    let x = 5;
    //x copise into func
    makes_copy(x);
    println!("{}",x)

}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}",some_integer);
}