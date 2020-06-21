
#[derive(Debug)]
enum Cell{
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(dead_code)]
pub fn vec_main(){

    let v = vec![1,2,3,4,5];
    for e in v.iter() {
        println!("{}", e);
    }
    let mut v : Vec<i32> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let e = &v[0];
    //v.push(3);
    println!("{} {:?}",e, v);

    //for e in v {
    for e in &v {

        println!("{}", e);
    }

    println!("{:?}", v);

    let v = vec![
        Cell::Int(0),
        Cell::Float(1.0),
        Cell::Text(String::from("Text"))
    ];
    for e in &v {
        match e {
            Cell::Int(v) => println!("{}", v),
            Cell::Float(v) => println!("{}", v),
            Cell::Text(v) => println!("{}", v),
        }
    }
    println!("{:?}", v);
}