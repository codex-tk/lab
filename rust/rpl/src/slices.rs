#[allow(dead_code)]
pub fn slices_main(){
    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("{} {}", hello, world);

    println!("{}", first_world(&s));
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}