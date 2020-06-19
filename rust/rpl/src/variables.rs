#[allow(dead_code)]
pub const MAX_POINTS : u32 = 100_000;

#[allow(dead_code)]
pub fn variables(){
    let mut x = 5;
    println!("The value x is {}" , x);
    x = 6;
    println!("The value x is {}" , x);
    let x = "shadow x";
    println!("The value x is {}" , x);

    let x = (500,2.0,'a');
    println!("{} {} {}", x.0, x.1, x.2);

    let x = [0,1,2,3,4,5];
    println!("{:?}", x);
}