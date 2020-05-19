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
    println!("{}",x);

    //func moves return to s1
    let s1 = gives_ownership();
    println!("{}",s1);

    //s2 gose into scope
    let s2 = String::from("GoodBay");
    //s2 moves into func 
    //and s3 gose into scope
    let s3 = takes_and_gives_back(s2);
    println!("{}",s3);

    let s1 = String::from("NiceToMeetYou");
    let(s2,len) = calculate_length(s1);
    println!("The length of {} is {}.",s2,len);
}

fn takes_ownership(some_string: String){
    println!("{}",some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}",some_integer);
}

fn gives_ownership()->String{
    let some_string = String::from("GoodBay");  
    some_string
}

fn takes_and_gives_back(a_strung: String)->String{
    a_strung
}

fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s, length)
}