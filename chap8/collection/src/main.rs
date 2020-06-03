enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //Vector
    let _v:Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];

    v.push(4);
    v.push(5);

    println!("{:?}",v);

    let third: &i32 = &v[2];
    println!("Third is {}.",third);

    let third: Option<&i32> = v.get(2);

    println!("{:?}",third);

    for i in &mut v{
        *i += 50;
    }

    for i in &v{
        println!("{}",i);
    }

    let _row = vec![
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
    
    //HashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Red"),String::from("Green")];
    let initial_scores = vec![10,50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let score = scores.get(&String::from("Red"));

    print!("{:?}",score);

    for (key, value) in &scores{
        println!("{},{}",key,value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}",scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{:?}",scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace(){
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);

}
