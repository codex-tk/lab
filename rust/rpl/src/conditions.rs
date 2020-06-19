#[allow(dead_code)]
pub fn condition_main(){
    let a = [0,1,2,3,4,5];
    for el in a.iter(){
        println!("{}", el);
    }

    for el in (1..5).rev() {
        println!("{}", el);
    }
}