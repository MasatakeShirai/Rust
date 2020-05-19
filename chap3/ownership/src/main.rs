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
}
