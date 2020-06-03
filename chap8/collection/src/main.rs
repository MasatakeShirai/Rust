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
}
