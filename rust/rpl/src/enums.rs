

enum IpAddr{
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message{
    Quit,
    Move{ x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){
        match self {
            Message::Quit => {
                println!("Quit Called")
            }
            Message::Move{ ref x, ref y } => {
                println!("Move Called {} {}", x, y)
            }
            _ => {}
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some(v+1),
        _ => None
    }
}

#[allow(dead_code)]
pub fn enums_main(){
    let f = IpAddr::V4(127,0,0,1);
    let s = IpAddr::V6(String::from("::1"));
    let m = Message::Quit;
    m.call();
    let m = Message::Move {x:10, y:20};
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    if let Some(v) = six {
        println!("{}", v);
    }

    if let Some(v) = none {
        println!("{}", v);
    }

}