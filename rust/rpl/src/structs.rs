#[allow(dead_code)]
pub fn structs_main(){
    let user = User{
        username: String::from("Name"),
        email: String::from("Email"),
        sign_in_count: 10,
        active: false
    };

    let username = String::from("Name");
    let email = String::from("Email");

    let user1 = User{
        email,
        username,
        sign_in_count: 32,
        active: true
    };

    println!("{} {}", user1.username, user1.email);

    let username = String::from("Name");
    let email = String::from("Email");
    let user2 = User{
        username,
        email,
        ..user1
    };

    println!("{} {}", user2.username, user2.email);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);

#[allow(dead_code)]
pub fn rectangles_main(){
    let length = 50;
    let width = 30;
    let rect = Rectangle{
        length: length,
        width: width,
    };
    println!("{} {:?}", rect.area(), rect);
}

#[derive(Debug)]
struct Rectangle{
    length: i32,
    width: i32,
}

impl Rectangle{
    fn area(&self) -> i32{
        self.length * self.width
    }
}