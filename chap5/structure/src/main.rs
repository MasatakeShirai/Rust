fn main() {
    let mut user1 = User{
        email: String::from("someone@example.ac.jp"),
        username: String::from("SomeoneName"),
        active: true,
        sign_in_count: 1,
    };

    print_user(&user1);

    user1.email = String::from("another@example.ac.jp");

    print_user(&user1);

    let user2 = buile_user(String::from("user2@ac.jp",), String::from("user2"));
    print_user(&user2);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("black:({},{},{})",black.0,black.1,black.2);
    println!("origin:({},{},{})",origin.0,origin.1,origin.2);
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn print_user(user:&User){
    println!("-------------------");
    print!("username:{}\nemail:{}\nsign_in_count:{}\nactive:{}\n"
            ,user.username,user.email,user.sign_in_count,user.active);
    println!("-------------------");
}

fn buile_user(email:String, username:String)->User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
