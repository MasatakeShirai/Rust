#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let lect1 = Rectangle{width:30, height:50};

    println!("Rect1 is {:#?}",lect1);

    println!(
        "The area of the rectangle is {} square pixels."
        ,lect1.area()
    );
}
