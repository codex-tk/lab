#[allow(dead_code)]
pub fn functions(){
    println!("functions");

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x {} y {}", x, y);
}

#[allow(dead_code)]
pub fn function_with(x: i32){
    println!("function {}", x);
}