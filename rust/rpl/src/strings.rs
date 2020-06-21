

pub fn strings_main(){
    let mut s = "to string".to_string();
    s.push_str("str");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s1 = s1 + &s2;
    println!("{} {}", s1, s2);

    let hello = "Здравствуйте";

    //let s = &hello[0];
    //let s = &hello[0..1];

    for c in hello.chars(){
        println!("{}", c);
    }

    for c in hello.bytes(){
        println!("{}", c);
    }
}