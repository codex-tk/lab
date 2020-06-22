
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

trait CustomIterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter{
    count: u32,
}

impl Counter{
    fn new() -> Counter {
        Counter{
            count: 0,
        }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.count;
        self.count += 1;
        if self.count < 6{
            Some(v)
        }else{
            None
        }
    }
}


pub fn main(){
    let v = vec![1,2,3];
    let mut v_iter = v.iter();
    //assert_eq!(v_iter.next(), Some(&1));
    //print_type_of(&(v_iter.next()));
    let v2: Vec<i32> = v_iter.map(|x| x + 1).collect();
    println!("{:?}", v2);
    let v3: Vec<i32> = v.into_iter().filter(|x| x < &3 ).collect();
    println!("{:?}", v3);
    let mut c = Counter::new();
    for v in c{
        println!("{}", v);
    }
}