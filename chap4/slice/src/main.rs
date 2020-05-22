fn main() {
    let mut s = String::from("hello world");
    let fword = first_word(&s);
    let sword = second_word(&s);
    
    println!("{},{}",fword,sword);

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}",hello,world);
    
    s = String::from("hello");

    let mut slice1 = &s[0..2];
    let mut slice2 = &s[..2];
    println!("{}, {}",slice1,slice2);
    
    let len = s.len();

    slice1 = &s[3..len];
    slice2 = &s[3..];
    println!("{}, {}",slice1,slice2);

    slice1 = &s[0..len];
    slice2 = &s[..len];
    println!("{}, {}",slice1,slice2);    
    
    s.clear();
}

fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s:&String)->&str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[i..];
        }    
    } 
    &s[..]
}
