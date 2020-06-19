
#[allow(dead_code)]
pub fn borrow_main(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
    take_ownership(s2);
    //println!("{}", s2);

    let x = 0;
    make_copy(x);
    println!("{}", x); // Copy trait

    let len = str_len(&s1); // borrow
    println!("{} {}", s1, len);

    let mut hello = s1.clone();
    strcat(&mut hello, " world");
    println!("{}", hello);

    let _immutable0 = &hello;
    let _immutable1 = &hello;
}

fn take_ownership(x: String){
    println!("{}", x);
}

fn make_copy(x: i32){
    println!("{}", x);
}

fn str_len(x: &String) -> usize{
    x.len()
}

fn strcat(s:&mut String, s1: &str ) {
    s.push_str(s1);
}