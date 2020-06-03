enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //Vector
    let v:Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    println!("Third is {}.",third);

    let third: Option<&i32> = v.get(2);

    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}",i);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("hello"))
    ];

    //String
    let data = "initial String";

    let s = data.to_string();
    println!("{}",s);

    let s = String::from("second string");
    println!("{}",s);

    let mut s = String::from("foo");
    let s1 = "bar";
    s.push_str(s1);
    println!("{}",s);
    println!("{}",s1);

    s.push('l');
    println!("{}",s);

    let s1 = String::from("Hello,");
    let s2 = String::from("World!");
    println!("{}",s1+&s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}",s1,s2,s3);
    println!("{}",s);        
    
}
