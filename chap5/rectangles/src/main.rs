#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let lect1 = Rectangle{width:30, height:50};

    println!("Rect1 is {:#?}",lect1);

    println!(
        "The area of the rectangle is {} square pixels."
        ,area(&lect1)
    );
}

fn area(rectangle: &Rectangle)->u32{
    rectangle.height * rectangle.width
}
